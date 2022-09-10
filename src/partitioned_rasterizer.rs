use dashmap::DashMap;

use crate::color;
use crate::rasterizer::*;
use crate::color::*;

use std::thread;
use std::thread::*;
use std::sync::Arc;



// If a bounding area in pixels is greater than this number, run in parallel instead
// Otherwise the extra setup is not worth the effort

#[derive(Copy, Clone)]
pub enum BoundingParallelThreshold {
	Always 	= 0,
	High 	= 40000,
	Medium 	= 80000,
	Low 	= 160000,
	VeryLow = 320000,
}

pub enum PartitionScheme {
	Full,
	Split2x1,
	Split1x2,
	Split2x2,
	Split3x1,
	Split3x2,
	Split3x3,
	Split4x4,
	Split5x5,
}

pub struct PartitionedRasterizer {
	pub rasterizer: Rasterizer,
	pub partitions: Vec<Rasterizer>,
	pub scheme: PartitionScheme,
	pub threshold: BoundingParallelThreshold,
}

/// A rasterizer that allows for parallel rendering by partioning the image into smaller pieces, usually by how many cores the current CPU has.
impl<'rast> PartitionedRasterizer {
	pub fn new(width: usize, height: usize) -> PartitionedRasterizer {
		let cpu_count = num_cpus::get();

		let mut scheme: PartitionScheme = match cpu_count {
			1 => { PartitionScheme::Full },
			2 => { if width > height { PartitionScheme::Split2x1 } else { PartitionScheme::Split1x2 }},
			4 => { PartitionScheme::Split2x2 },
			6 => { PartitionScheme::Split3x2 },
			8 => { PartitionScheme::Split3x3 },
			10 => { PartitionScheme::Split4x4 },
			12 => { PartitionScheme::Split4x4 },
			16 => { PartitionScheme::Split4x4 },
			24 => { PartitionScheme::Split5x5 },
			_ => { PartitionScheme::Split4x4 }
		};

		if cpu_count > 16 {
			scheme = PartitionScheme::Split5x5;
		}

		let mut pr = PartitionedRasterizer {
			rasterizer: Rasterizer::new(width, height),
			partitions:  Vec::new(),
			scheme,
			threshold: BoundingParallelThreshold::Always,
		};

		pr.generate_partitions();
		pr
	}

	// For some reason Result doesn't work here????
	pub fn new_from_image(path_to: &str) -> PartitionedRasterizer {
		match lodepng::decode32_file(path_to) {
			Ok(image) => {
				//println!("Image: {}, Res: {} x {}, Size: {}B", path_to, image.width, image.height, image.buffer.len());
				let buffer_new: Vec<u8> =  image.buffer.as_bytes().to_vec();
                use rgb::*;

				let mut pr = PartitionedRasterizer::new(image.width, image.height);
				pr.rasterizer.color = buffer_new;
				pr.generate_partitions();

				pr
			},
			Err(reason) => {
				println!("ERROR - IMAGE: Could not load {} | {}", path_to, reason);
				PartitionedRasterizer::new(1, 1)
			}
		}
	}

	pub fn clear(&mut self) {
		self.rasterizer.clear();
	}

	pub fn clear_color(&mut self, color: Color) {
		self.rasterizer.clear_color(color);
	}

	pub fn set_draw_mode(&mut self, mode: DrawMode) {
		self.rasterizer.set_draw_mode(mode);
	}

	pub fn resize(&mut self, width: usize, height: usize) {
		self.rasterizer.resize(width, height);
		self.generate_partitions();
	}

	pub fn pset(&mut self, x: i64, y: i64, color: Color) {
		self.rasterizer.pset(x, y, color);
	}

	pub fn pget(&mut self, x: i64, y: i64) {
		self.rasterizer.pget(x, y);
	}

	// Too simple to parallelize
	pub fn pline(&mut self, x0: i64, y0: i64, x1: i64, y1: i64, color: Color) {
		self.rasterizer.pline(x0, y0, x1, y1, color);
	}

	pub fn prectangle(&mut self, filled: bool, x: i64, y: i64, width: i64, height: i64, color: Color) {
		let total_area = width * height;

		// Run in parallel
		if filled && total_area >= self.threshold as i64 {
			let mut join_handles: Vec<JoinHandle<Rasterizer>> = Vec::new();
			
			for part in &self.partitions {
				let mut part_clone = part.clone();
				let handle = thread::spawn( move || {
					part_clone.prectangle(filled, x, y, width, height, color);

					part_clone
				});
				join_handles.push(handle);
			}

			for handle in join_handles {
				let part_return = handle.join();
				if part_return.is_ok() {
					self.rasterizer.blit(&part_return.unwrap());
				} else {
					println!("ERROR - THREAD PANIC: Partition failed in prectangle function!")
				}
			}
		} else { // Just lines
			self.rasterizer.prectangle(filled, x, y, width, height, color);
		}
	}

	pub fn pcircle(&mut self, filled: bool, xc: i64, yc: i64, radius: i64, color: Color) {
		let total_area = std::f64::consts::PI * (radius * radius) as f64 ;

		// Run in parallel
		if total_area >= self.threshold as i64 as f64 {
			scope(|s| {
				let mut join_handles: Vec<ScopedJoinHandle<&mut Rasterizer>> = Vec::new();
			
				for part in & mut self.partitions {
					//let mut part_clone = part.clone();
	
					let rx = xc - part.offset_x as i64;
					let ry = yc - part.offset_y as i64;
	
					let handle = s.spawn(move || {
	
						part.pcircle(filled, rx, ry, radius, color);
	
						part
					});
					join_handles.push(handle);
				}
	
				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						self.rasterizer.blit(&part_return.unwrap());
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pcircle function!")
					}
				}
			})
			
		} else {
			self.rasterizer.pcircle(filled, xc, yc, radius, color);
		}
	}

	pub fn pimgmtx(&mut self, image: &Rasterizer, x: f64, y: f64, rotation: f64, scale_x: f64, scale_y: f64, offset_x: f64, offset_y: f64) {

		let width = image.width;
		let height = image.height;
		// Approximate area, can be bigger depending on rotation
		let total_area = (width as f64 * scale_x) * (height as f64 * scale_y);

		// Run in parallel
		if total_area >= self.threshold as i64 as f64 {
			scope(|s| {
				let mut join_handles: Vec<ScopedJoinHandle<Rasterizer>> = Vec::new();
			
				for part in & mut self.partitions {
					let mut part_clone = part.clone();
	
					let rx = x - part.offset_x as f64;
					let ry = y - part.offset_y as f64;
	
					let handle = s.spawn(move || {
	
						part_clone.pimgmtx(image, rx, ry, rotation, scale_x, scale_y, offset_x, offset_y);
	
						part_clone
					});
					join_handles.push(handle);
				}
	
				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						self.rasterizer.blit(&part_return.unwrap());
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pimgmtx function!")
					}
				}
			})
			
		} else {
			self.rasterizer.pimgmtx(&image, x, y, rotation, scale_x, scale_y, offset_x, offset_y);
		}
		
	}

	fn generate_partitions(&mut self) {
		match self.scheme {
			PartitionScheme::Full => { self.partition_full(); },
			PartitionScheme::Split1x2 => { self.partition_split_vertical(); }
			PartitionScheme::Split2x1 => { self.partition_split_horizontal(); }
			PartitionScheme::Split2x2 => { self.partition_split_2x2(); }
			PartitionScheme::Split3x3 => { self.partition_split_3x3(); }
			PartitionScheme::Split3x2 => { self.partition_split_3x2(); }
			PartitionScheme::Split3x1 => { self.partition_split_3x1(); }
			PartitionScheme::Split4x4 => { self.partition_split_4x4(); }
			PartitionScheme::Split5x5 => { self.partition_split_5x5(); }
			_ => {},
		}
	}

	pub fn draw_debug_view(&mut self) {
		for part in &self.partitions {
			self.rasterizer.pline(
				part.offset_x as i64, 
				part.offset_y as i64, 
				(part.offset_x + part.width)  as i64, 
				part.offset_y  as i64, 
				Color::new(255, 0, 255, 255)
			);

			self.rasterizer.pline(
				part.offset_x as i64, 
				part.offset_y as i64, 
				part.offset_x  as i64, 
				(part.offset_y + part.height) as i64, 
				Color::new(255, 0, 255, 255)
			);
		}
	}

	fn partition_full(&mut self) {
		self.partitions.clear();
	}

	fn partition_split_vertical(&mut self) {
		let mut left_half: Rasterizer = Rasterizer::new(self.rasterizer.width / 2, self.rasterizer.height);
		left_half.offset_x = 0;
		left_half.offset_y = 0;

		let mut right_half: Rasterizer = Rasterizer::new(self.rasterizer.width / 2, self.rasterizer.height);
		right_half.offset_x = self.rasterizer.width / 2;
		right_half.offset_y = 0;

		self.partitions.clear();
		self.partitions.push(left_half);
		self.partitions.push(right_half);
	}

	fn partition_split_horizontal(&mut self) {
		let mut top_half: Rasterizer = Rasterizer::new(self.rasterizer.width, self.rasterizer.height / 2);
		top_half.offset_x = 0;
		top_half.offset_y = 0;

		let mut bottom_half: Rasterizer = Rasterizer::new(self.rasterizer.width, self.rasterizer.height / 2);
		bottom_half.offset_x = 0;
		bottom_half.offset_y = self.rasterizer.height / 2;

		self.partitions.clear();
		self.partitions.push(top_half);
		self.partitions.push(bottom_half);
	}

	fn partition_split_2x2(&mut self) {
		let cell_x = self.rasterizer.width / 2;
		let cell_y = self.rasterizer.height / 2;

		for y in 0..2 {
			for x in 0..2 {
				let mut r = Rasterizer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}

	fn partition_split_3x1(&mut self) {
		let cell_x = self.rasterizer.width / 3;
		let cell_y = self.rasterizer.height;

		self.partitions.clear();

		for y in 0..3 {
			for x in 0..1 {
				let mut r = Rasterizer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}

	fn partition_split_3x2(&mut self) {
		let cell_x = self.rasterizer.width / 3;
		let cell_y = self.rasterizer.height / 2;

		self.partitions.clear();

		for y in 0..3 {
			for x in 0..2 {
				let mut r = Rasterizer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}

	fn partition_split_3x3(&mut self) {
		let cell_x = self.rasterizer.width / 3;
		let cell_y = self.rasterizer.height / 3;

		self.partitions.clear();

		for y in 0..3 {
			for x in 0..3 {
				let mut r = Rasterizer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}

	fn partition_split_4x4(&mut self) {
		let cell_x = self.rasterizer.width / 4;
		let cell_y = self.rasterizer.height / 4;

		self.partitions.clear();

		for y in 0..4 {
			for x in 0..4 {
				let mut r = Rasterizer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}

	fn partition_split_5x5(&mut self) {
		let cell_x = self.rasterizer.width / 5;
		let cell_y = self.rasterizer.height / 5;

		self.partitions.clear();

		for y in 0..5 {
			for x in 0..5 {
				let mut r = Rasterizer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}
}
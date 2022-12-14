use crate::rasterizer::*;
use crate::color::*;

use std::thread::*;

use crate::vector2::*;

// If a bounding area in pixels is greater than this number, run in parallel instead
// Otherwise the extra setup is not worth the effort
#[derive(Copy, Clone)]
pub enum BoundingParallelThreshold {
	Always 	= 0,
	High 	= 8192,
	Medium 	= 16384,
	Low 	= 32768,
	VeryLow = 65536,
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
	Split8x8,
}

pub struct PartitionedRasterizer {
	pub rasterizer: Rasterizer,
	pub partitions: Vec<Rasterizer>,
	pub scheme: PartitionScheme,
	pub threshold: BoundingParallelThreshold,
}

/// A rasterizer that allows for parallel rendering by partioning the image into smaller pieces, usually by how many cores the current CPU has.
impl PartitionedRasterizer {
	pub fn new(width: usize, height: usize, cores: usize) -> PartitionedRasterizer {
	
		let mut pr = PartitionedRasterizer {
			rasterizer: Rasterizer::new(width, height),
			partitions:  Vec::new(),
			scheme: PartitionScheme::Full,
			threshold: BoundingParallelThreshold::High,
		};

		pr.set_core_limit(cores);
		pr
	}

	// For some reason Result doesn't work here????
	pub fn new_from_image(path_to: &str) -> PartitionedRasterizer {
		match lodepng::decode32_file(path_to) {
			Ok(image) => {
				//println!("Image: {}, Res: {} x {}, Size: {}B", path_to, image.width, image.height, image.buffer.len());
				let buffer_new: Vec<u8> =  image.buffer.as_bytes().to_vec();
                use rgb::*;

				let mut pr = PartitionedRasterizer::new(image.width, image.height, 0);
				pr.rasterizer.color = buffer_new;
				pr.generate_partitions();

				pr
			},
			Err(reason) => {
				println!("ERROR - IMAGE: Could not load {} | {}", path_to, reason);
				PartitionedRasterizer::new(1, 1, 1)
			}
		}
	}

	pub fn clear(&mut self) {
		self.rasterizer.clear();
		for part in &mut self.partitions {
			part.clear();
		}
	}

	pub fn clear_color(&mut self, color: Color) {
		self.rasterizer.clear_color(color);
		for part in &mut self.partitions {
			part.clear_color(color);
		}
	}

	pub fn set_core_limit(&mut self, cores: usize) {
		let cpu_count = if cores == 0 { num_cpus::get() } else { cores };

		let mut is_unknown: bool = false;

		let mut scheme: PartitionScheme = match cpu_count {
			1 => { PartitionScheme::Full },
			2 => { if self.rasterizer.height > self.rasterizer.width { PartitionScheme::Split2x1 } else { PartitionScheme::Split1x2 }},
			3 => { PartitionScheme::Split3x1 }
			4 => { PartitionScheme::Split2x2 },
			5 => { PartitionScheme::Split2x2 },
			6 => { PartitionScheme::Split3x2 },
			8 => { PartitionScheme::Split3x3 },
			10 => { PartitionScheme::Split3x3 },
			12 => { PartitionScheme::Split4x4 },
			16 => { PartitionScheme::Split4x4 },
			20 => { PartitionScheme::Split5x5 },
			24 => { PartitionScheme::Split5x5 },
			_ => { is_unknown = true; PartitionScheme::Split4x4 }
		};

		if is_unknown {
			if cpu_count > 24 {
				scheme = PartitionScheme::Split8x8;
			}
		}
		self.scheme = scheme;
		self.generate_partitions();
	}

	pub fn set_draw_mode(&mut self, mode: DrawMode) {
		self.rasterizer.set_draw_mode(mode);
		for part in &mut self.partitions {
			part.set_draw_mode(mode);
		}
	}

	pub fn set_tint(&mut self, color: Color) {
		self.rasterizer.tint = color;
		for part in &mut self.partitions {
			part.tint = color;
		}
	}

	pub fn set_opacity(&mut self, opacity: u8) {
		self.rasterizer.opacity = opacity;
		for part in &mut self.partitions {
			part.opacity = opacity;
		}
	}

	pub fn set_camera_position(&mut self, x: f64, y: f64) {
		self.rasterizer.camera_position = Vector2::new(x, y);
		for part in &mut self.partitions {
			part.camera_position = Vector2::new(x, y);
		}
	}

	pub fn set_camera_rotation(&mut self, rotation: f64) {
		self.rasterizer.camera_rotation = rotation;
		for part in &mut self.partitions {
			part.camera_rotation = rotation;
		}
	}

	pub fn set_camera_scale(&mut self, x: f64, y: f64) {
		self.rasterizer.camera_scale = Vector2::new(x, y);
		for part in &mut self.partitions {
			part.camera_scale = Vector2::new(x, y);
		}
	}

	pub fn update_camera(&mut self) {
		self.rasterizer.update_camera();
		for part in &mut self.partitions {
			part.update_camera();
		}
	}

	pub fn resize(&mut self, width: usize, height: usize) {
		self.rasterizer.resize(width, height);
		self.generate_partitions();
	}

	pub fn blit(&mut self, image: &Rasterizer, x: i64, y: i64) {
		self.rasterizer.blit(image, x, y);
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
			scope(|s| {
				let mut join_handles: Vec<ScopedJoinHandle<&mut Rasterizer>> = Vec::new();
			
				for part in & mut self.partitions {
	
					let rx = x - part.offset_x as i64;
					let ry = y - part.offset_y as i64;
	
					let handle = s.spawn(move || {
	
						part.prectangle(filled, rx, ry, width, height, color);
	
						part
					});
					join_handles.push(handle);
				}
	
				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						let part = part_return.unwrap();
						self.rasterizer.blit(&part, part.offset_x as i64, part.offset_y as i64);
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pcircle function!")
					}
				}
			})
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
						let part = part_return.unwrap();
						self.rasterizer.blit(&part, part.offset_x as i64, part.offset_y as i64);
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pcircle function!")
					}
				}
			})
			
		} else {
			self.rasterizer.pcircle(filled, xc, yc, radius, color);
		}
	}

	pub fn pimg(&mut self, image: &Rasterizer, x: i64, y: i64) {

		let width = image.width;
		let height = image.height;
		// Approximate area, can be bigger depending on rotation
		let total_area = (width as i64) * (height as i64);

		// Run in parallel
		if total_area >= self.threshold as i64 {
			scope(|s| {
				let mut join_handles: Vec<ScopedJoinHandle<&mut Rasterizer>> = Vec::new();
			
				for part in &mut self.partitions {

					let rx = x - part.offset_x as i64;
					let ry = y - part.offset_y as i64;


					let handle = s.spawn(move || {
	
						part.pimg(image, rx, ry);
	
						part
					});
					join_handles.push(handle);
				}
	
				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						let part = part_return.unwrap();
						self.rasterizer.blit(&part, part.offset_x as i64, part.offset_y as i64);
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pimg function!")
					}
				}
			})
			
		} else {
			self.rasterizer.pimg(&image, x, y);
		}
		
	}

	pub fn pimgrect(&mut self, image: &Rasterizer, x: i64, y: i64, ix: i64, iy: i64, iw: i64, ih: i64) {

		let width = image.width;
		let height = image.height;
		// Approximate area, can be bigger depending on rotation
		let total_area = (width as i64) * (height as i64);

		// Run in parallel
		if total_area >= self.threshold as i64 {
			scope(|s| {
				let mut join_handles: Vec<ScopedJoinHandle<&mut Rasterizer>> = Vec::new();
			
				for part in &mut self.partitions {

					let rx = x - part.offset_x as i64;
					let ry = y - part.offset_y as i64;


					let handle = s.spawn(move || {
	
						part.pimgrect(image, rx, ry, ix, iy, iw, ih);
	
						part
					});
					join_handles.push(handle);
				}
	
				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						let part = part_return.unwrap();
						self.rasterizer.blit(&part, part.offset_x as i64, part.offset_y as i64);
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pimg function!")
					}
				}
			})
			
		} else {
			self.rasterizer.pimg(&image, x, y);
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

				let mut join_handles: Vec<ScopedJoinHandle<&mut Rasterizer>> = Vec::new();

				// First pass: Find all regions that contain the image
				for part in &mut self.partitions {
					let rx = x - part.offset_x as f64;
					let ry = y - part.offset_y as f64;
					
					let handle = s.spawn( move || {
						
						part.pimgmtx(image, rx, ry, rotation, scale_x, scale_y, offset_x, offset_y);
	
						part
					});
					join_handles.push(handle);
				}

				for handle in join_handles {
					let part_return = handle.join();
					if part_return.is_ok() {
						let part = part_return.unwrap();
						self.rasterizer.blit(&part, part.offset_x as i64, part.offset_y as i64);
					} else {
						println!("ERROR - THREAD PANIC: Partition failed in pimgmtx function!")
					}
				}
				
				//self.rasterizer.prectangle(false, rsx as i64, rsy as i64, (rex - rsx) as i64, (rey - rsy) as i64, Color::blue());
			})
			
		} else {
			self.rasterizer.pimgmtx(&image, x, y, rotation, scale_x, scale_y, offset_x, offset_y);
		}
		
	}


	fn generate_partitions(&mut self) {
		self.partitions.clear();
		/*let mut divx = min_pixel_size;

		for i in min_pixel_size..self.rasterizer.width {
			if (self.rasterizer.width % i) == 0 {
				divx = i;
				break;
			} 
		}

		let mut divy = min_pixel_size;

		for i in min_pixel_size..self.rasterizer.height {
			if (self.rasterizer.height % i) == 0 {
				divy = i;
				break;
			} 
		}

		let (cx, cy) = (self.rasterizer.width / divx, self.rasterizer.height / divy);

		for y in 0..cy {
			for x in 0..cx {
				let mut r = Rasterizer::new(divx, divy);
				r.offset_x = x * divx;
				r.offset_y = y * divy;
				self.partitions.push(r);
			}
		}*/

		match self.scheme {
			PartitionScheme::Full => { self.partition_full(); },
			PartitionScheme::Split1x2 => { self.partition_split_vertical(); },
			PartitionScheme::Split2x1 => { self.partition_split_horizontal(); },
			PartitionScheme::Split2x2 => { self.partition_split_2x2(); },
			PartitionScheme::Split3x3 => { self.partition_split_3x3(); },
			PartitionScheme::Split3x2 => { self.partition_split_3x2(); },
			PartitionScheme::Split3x1 => { self.partition_split_3x1(); },
			PartitionScheme::Split4x4 => { self.partition_split_4x4(); },
			PartitionScheme::Split5x5 => { self.partition_split_5x5(); },
			PartitionScheme::Split8x8 => { self.partition_split_8x8(); },
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
		self.partitions.push(Rasterizer::new(self.rasterizer.width, self.rasterizer.height));
	}

	fn partition_split_vertical(&mut self) {
		let mut left_half: Rasterizer = Rasterizer::new(self.rasterizer.width / 2, self.rasterizer.height);
		left_half.offset_x = 0;
		left_half.offset_y = 0;

		let mut right_half: Rasterizer = Rasterizer::new(self.rasterizer.width / 2, self.rasterizer.height);
		right_half.offset_x = self.rasterizer.width / 2;
		right_half.offset_y = 0;

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

		let mut r = Rasterizer::new(cell_x, cell_y);
		r.offset_x = 0;
		r.offset_y = 0;
		self.partitions.push(r);

		let mut r = Rasterizer::new(cell_x, cell_y);
		r.offset_x = cell_x;
		r.offset_y = 0;
		self.partitions.push(r);

		let mut r = Rasterizer::new(cell_x, cell_y);
		r.offset_x = cell_x * 2;
		r.offset_y = 0;
		self.partitions.push(r);
	}

	fn partition_split_3x2(&mut self) {
		let cell_x = self.rasterizer.width / 3;
		let cell_y = self.rasterizer.height / 2;

		for y in 0..2 {
			for x in 0..3 {
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

		for y in 0..5 {
			for x in 0..5 {
				let mut r = Rasterizer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}

	fn partition_split_8x8(&mut self) {
		let cell_x = self.rasterizer.width / 8;
		let cell_y = self.rasterizer.height / 8;

		for y in 0..8 {
			for x in 0..8 {
				let mut r = Rasterizer::new(cell_x, cell_y);
				r.offset_x = cell_x * x;
				r.offset_y = cell_y * y;
				self.partitions.push(r);
			}
		}
	}
}
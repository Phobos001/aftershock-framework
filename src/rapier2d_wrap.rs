use std::rc::Rc;

use dashmap::DashMap;
use rapier2d_f64::prelude::*;
use rapier2d_f64::na as na;

pub struct RapierWorld2D {
	pub handles_collider: DashMap<String, ColliderHandle>,
	pub handles_rigidbody: DashMap<String, RigidBodyHandle>,

	pub rigid_body_set: RigidBodySet,
	pub collider_set: ColliderSet,

	// Scaling factor to make physics calculations easier
	pub pixels_per_meter: f64,

	pub gravity: na::Vector2<f64>,
	pub integration_parameters: IntegrationParameters,
	pub physics_pipeline: PhysicsPipeline,
	pub island_manager: IslandManager,
	pub broad_phase: BroadPhase,
	pub narrow_phase: NarrowPhase,
	pub impulse_joint_set: ImpulseJointSet,
	pub multibody_joint_set: MultibodyJointSet,
	pub ccd_solver: CCDSolver,
	pub physics_hooks: (),
	pub event_handler: (),
}

impl RapierWorld2D {
	pub fn new(target_dt: f64) -> RapierWorld2D {
		let gravity = vector![0.0, 0.0];
		let mut integration_parameters = IntegrationParameters::default();

		integration_parameters.dt = target_dt;

		let physics_pipeline = PhysicsPipeline::new();
		let island_manager = IslandManager::new();
		let broad_phase = BroadPhase::new();
		let narrow_phase = NarrowPhase::new();
		let impulse_joint_set = ImpulseJointSet::new();
		let multibody_joint_set = MultibodyJointSet::new();
		let ccd_solver = CCDSolver::new();
		let physics_hooks = ();
		let event_handler = ();

		RapierWorld2D { 
			handles_collider: DashMap::new(),
			handles_rigidbody: DashMap::new(),
			rigid_body_set: RigidBodySet::new(),
			collider_set: ColliderSet::new(),
			pixels_per_meter: 1.0,
			gravity,
			integration_parameters,
			physics_pipeline,
			island_manager,
			broad_phase,
			narrow_phase,
			impulse_joint_set,
			multibody_joint_set,
			ccd_solver,
			physics_hooks,
			event_handler,
		}
	}

	pub fn update(&mut self, delta: f64) {
		self.physics_pipeline.step(
			&self.gravity,
			&self.integration_parameters,
			&mut self.island_manager,
			&mut self.broad_phase,
			&mut self.narrow_phase,
			&mut self.rigid_body_set,
			&mut self.collider_set,
			&mut self.impulse_joint_set,
			&mut self.multibody_joint_set,
			&mut self.ccd_solver,
			&self.physics_hooks,
			&self.event_handler,
		);
	}

	pub fn add_static_rect(&mut self, x: f64, y: f64, width: f64, height: f64) {
		let collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0).translation(vector![x, y]).build();
		let _ = self.collider_set.insert(collider);
		
	}

	pub fn add_dynamic_rect(&mut self,x: f64, y: f64, width: f64, height: f64) {
		let collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0).build();
		let rigidbody = self.rigid_body_set.insert(RigidBodyBuilder::dynamic().translation(vector![x, y]).build());
		let _ = self.collider_set.insert_with_parent(collider, rigidbody, &mut self.rigid_body_set);

	}

	pub fn add_static_rect_handle(&mut self, name: String, x: f64, y: f64, width: f64, height: f64) {
		let collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0).translation(vector![x, y]).build();
		let handle = self.collider_set.insert(collider);
		self.handles_collider.insert(name, handle);
	}

	pub fn add_dynamic_rect_handle(&mut self, name: String, x: f64, y: f64, width: f64, height: f64) {
		let collider = ColliderBuilder::cuboid(width / 2.0, height / 2.0).build();
		let rigidbody = self.rigid_body_set.insert(RigidBodyBuilder::dynamic().translation(vector![x, y]).build());
		let handle = self.collider_set.insert_with_parent(collider, rigidbody, &mut self.rigid_body_set);
		self.handles_collider.insert(name, handle);
	}
}
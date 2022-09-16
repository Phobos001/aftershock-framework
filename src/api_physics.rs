use rapier2d_f64::prelude::*;
use rapier2d_f64::na as na;

pub struct RapierWorld2D {
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
}
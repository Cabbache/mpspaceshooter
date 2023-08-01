use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use std::f32::consts::{PI};
use serde::{Serialize};

const ACCELERATION: f32 = 200.0; //player acceleration
const PROPEL_DIRECTION: f32 = -PI/2.0;
const RADIANS_PER_SECOND: f32 = PI; //player rotation speed
const G: f32 = 20.0; //Gravitational constant

const TIMESTEP_FPS: f32 = 8.0; //around 20 is good
const DRAG: f32 = 0.94; //velocity is multiplied by this every second

//Calculated
const TIMESTEP: f32 = 1f32 / TIMESTEP_FPS;
lazy_static! {
	static ref DRAGSTEP: f32 = DRAG.powf(1f32 / TIMESTEP_FPS);
}

pub const BODIES: [Body; 1] = [
	Body {
		pos: Vector{
			x: 0.0,
			y: 0.0,
		},
		radius: 300.0,
	},
];

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Trajectory{
	pub propelling: bool,
	pub pos: Vector,
	pub vel: Vector,
	pub spin: f32,
	pub spin_direction: i8, //-1,0,1
	pub time: f32, //TODO move time outside trajectory?
}

//TODO make cos/sin faster by storing the results
#[wasm_bindgen]
impl Trajectory {
	pub fn live(&self) -> Trajectory{
		let mut result = self.clone();
		let spin_speed = (result.spin_direction as f32) * RADIANS_PER_SECOND;
		let seconds = current_time() - result.time;
		let ctr = (seconds / TIMESTEP) as u32;
		for _ in 1..=ctr {
			result.pos.x += result.vel.x * TIMESTEP;
			result.pos.y += result.vel.y * TIMESTEP;
			for body in BODIES {
				let pull = body.pull(&result.pos);
				result.vel.x += pull.x * TIMESTEP;
				result.vel.y += pull.y * TIMESTEP;
			}
			result.spin += spin_speed * TIMESTEP;
			if result.propelling {
				result.vel.x += (result.spin + PROPEL_DIRECTION).cos()*ACCELERATION * TIMESTEP;
				result.vel.y += (result.spin + PROPEL_DIRECTION).sin()*ACCELERATION * TIMESTEP;
			}
			//result.vel.x *= *DRAGSTEP;
			//result.vel.y *= *DRAGSTEP;
		}
		let increment = TIMESTEP * (ctr as f32);
		result.time += increment;
		result
	}

	//live rotation doesnt need mutation
	pub fn live_rot(&self) -> f32 {
		let seconds = current_time() - self.time;
		(self.spin_direction as f32) * seconds * RADIANS_PER_SECOND + self.spin
	}

	pub fn update(&mut self){
		*self = self.live();
	}

	pub fn update_rotation(&mut self, new_direction: i8){
		self.update();
		self.spin_direction = new_direction;
	}

	pub fn update_propulsion(&mut self, on: bool){
		self.update();
		self.propelling = on;
	}
}

#[derive(Serialize, Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct Vector{
	pub x: f32,
	pub y: f32,
}

#[derive(Serialize, Clone, Debug)]
#[wasm_bindgen]
pub struct Body{
	pub pos: Vector,
	pub radius: f32,
}

#[wasm_bindgen]
impl Body {
	//returns the acceleration imposed by itself at a point
	fn pull(&self, pos: &Vector) -> Vector{
		let xdiff = self.pos.x - pos.x;
		let ydiff = self.pos.y - pos.y;
		let powsum = xdiff.powf(2.0) + ydiff.powf(2.0);
		let mag = G * self.mass() / powsum;
		let dist = powsum.sqrt();
		Vector{
			x: mag * xdiff / dist,
			y: mag * ydiff / dist,
		}
	}

	fn mass(&self) -> f32 {
		self.radius*self.radius*PI
	}
}

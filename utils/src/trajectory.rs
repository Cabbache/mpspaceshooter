use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use std::f32::consts::{PI};
use serde::{Deserialize,Serialize};
use bincode::{serialize, deserialize};
use base64::{engine::general_purpose, Engine as _};

pub const PLAYER_RADIUS: f32 = 25.0;
const PISTOL_REACH: f32 = 500.0; //players have circular hitbox
const DOME_RADIUS: f32 = 6000.0;
const ACCELERATION: f32 = 200.0; //player acceleration
const PROPEL_DIRECTION: f32 = -PI/2.0;
const RADIANS_PER_SECOND: f32 = PI; //player rotation speed
const G: f32 = 2000.0; //Gravitational constant

const TIMESTEP_FPS: u32 = 8; //around 20 is good
//const DRAG: f32 = 0.94; //velocity is multiplied by this every second

//Calculated
const TIMESTEP_MILLIS: u32 = 1000 / TIMESTEP_FPS;
const TIMESTEP_SECS: f32 = 1f32 / TIMESTEP_FPS as f32;
//lazy_static! {
//	static ref DRAGSTEP: f32 = DRAG.powf(1f32 / TIMESTEP_FPS as f32);
//}

pub const BODIES: [Body; 20] = [
  Body {
    pos: Vector{
      x: 0.0,
      y: 0.0,
    },  
    radius: 100.0,
  },  
  Body {
    pos: Vector{
      x: 500.0,
      y: 300.0,
    },  
    radius: 30.0,
  },
  Body {
    pos: Vector{
      x: -1000.0,
      y: 2000.0,
    },
    radius: 25.0,
  },
  Body {
    pos: Vector{
      x: 1500.0,
      y: -500.0,
    },
    radius: 20.0,
  },
  Body {
    pos: Vector{
      x: -500.0,
      y: -1500.0,
    },
    radius: 35.0,
  },
  Body {
    pos: Vector{
      x: -2500.0,
      y: 1000.0,
    },
    radius: 40.0,
  },
  Body {
    pos: Vector{
      x: 2000.0,
      y: -2000.0,
    },
    radius: 15.0,
  },
  Body {
    pos: Vector{
      x: -1500.0,
      y: -2500.0,
    },
    radius: 30.0,
  },
  Body {
    pos: Vector{
      x: 2500.0,
      y: 1500.0,
    },
    radius: 20.0,
  },
  Body {
    pos: Vector{
      x: 3000.0,
      y: 0.0,
    },
    radius: 35.0,
  },
	Body {
    pos: Vector{
      x: -2800.0,
      y: 600.0,
    },
    radius: 20.0,
  },
  Body {
    pos: Vector{
      x: 1500.0,
      y: 2800.0,
    },
    radius: 40.0,
  },
  Body {
    pos: Vector{
      x: 500.0,
      y: -2500.0,
    },
    radius: 35.0,
  },
  Body {
    pos: Vector{
      x: 2000.0,
      y: -1200.0,
    },
    radius: 30.0,
  },
  Body {
    pos: Vector{
      x: -2200.0,
      y: -1800.0,
    },
    radius: 40.0,
  },
  Body {
    pos: Vector{
      x: -1200.0,
      y: 1500.0,
    },
    radius: 35.0,
  },
  Body {
    pos: Vector{
      x: 2800.0,
      y: -800.0,
    },
    radius: 30.0,
  },
  Body {
    pos: Vector{
      x: -600.0,
      y: -2800.0,
    },
    radius: 40.0,
  },
  Body {
    pos: Vector{
      x: 800.0,
      y: 2200.0,
    },
    radius: 25.0,
  },
  Body {
    pos: Vector{
      x: -1800.0,
      y: 500.0,
    },
    radius: 20.0,
  },
];

#[derive(Debug, Clone, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct Trajectory{
	pub propelling: bool,
	pub pos: Vector,
	pub vel: Vector,
	pub spin: f32,
	pub spin_direction: i8, //-1,0,1
	pub time: u64,
	pub collision: bool,
}

impl Trajectory {
	pub fn to_b64(&self) -> String {
		general_purpose::STANDARD.encode(serialize(&self).unwrap())
	}

	pub fn pull_sum(pos: &Vector) -> Vector {
		let mut pull = Vector{
			x: 0.0,
			y: 0.0,
		};
		for body in BODIES {
			let bod_pull = body.pull(&pos);
			pull.x += bod_pull.x;
			pull.y += bod_pull.y;
		}
		pull.x *= TIMESTEP_SECS;
		pull.y *= TIMESTEP_SECS;
		pull
	}

	fn step(&mut self) -> bool{
		if self.collision {
			return false;
		}
		let next_pos = Vector{
			x: self.pos.x + self.vel.x*TIMESTEP_SECS,
			y: self.pos.y + self.vel.y*TIMESTEP_SECS,
		};
		if next_pos.x.powi(2) + next_pos.y.powi(2) > DOME_RADIUS.powi(2) {
			self.vel.reflect(&self.pos);
			self.vel.x *= 0.5;
			self.vel.y *= 0.5;
		} else {
			self.pos = next_pos;
		}
		let pull = Trajectory::pull_sum(&self.pos);
		self.vel.x += pull.x;
		self.vel.y += pull.y;
		self.spin += (self.spin_direction as f32) * RADIANS_PER_SECOND * TIMESTEP_SECS;
		if self.propelling {
			self.vel.x += (self.spin + PROPEL_DIRECTION).cos()*ACCELERATION * TIMESTEP_SECS;
			self.vel.y += (self.spin + PROPEL_DIRECTION).sin()*ACCELERATION * TIMESTEP_SECS;
		}
		self.collision = self.collides();
		true
	}
}

#[wasm_bindgen]
impl Trajectory {

	#[wasm_bindgen(constructor)]
	pub fn from_b64(data: String) -> Trajectory {
		deserialize(&general_purpose::STANDARD.decode(&data).unwrap()).unwrap()
	}

	//euler's method
	pub fn advance(&mut self, time: u64) -> bool {
		let elapsed = (time - self.time) as u32;
		let ctr = elapsed / TIMESTEP_MILLIS;
		let mut chpos = false;
		for _ in 1..=ctr {
			if !self.step() {
				return chpos;
			}
			chpos = true;
		}
		self.time += (TIMESTEP_MILLIS * ctr) as u64;
		true
	}

	//these 2, only for wasm
	pub fn set_rotation(&mut self, new_direction: i8) {
		self.spin_direction = new_direction;
	}

	pub fn set_propulsion(&mut self, on: bool) {
		self.propelling = on;
	}

	pub fn update_rotation(&mut self, new_direction: i8, time: u64){
		self.advance(time);
		self.spin_direction = new_direction;
	}

	pub fn update_propulsion(&mut self, on: bool, time: u64){
		self.advance(time);
		self.propelling = on;
	}

	//TODO consider when velocity exceeds radius, use line_intersects_circle?
	fn collides(&self) -> bool {
		for body in BODIES{
			if body.collides(&self.pos){
				return true;
			}
		}
		return false;
	}

	pub fn hash_str(&self) -> String {
		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);
		format!("{:x}", hasher.finish())
	}
}

impl Hash for Trajectory {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.propelling.hash(state);
		self.pos.hash(state);
		self.vel.hash(state);
		self.spin.to_bits().hash(state); // Manually hash f32 field
		self.spin_direction.hash(state);
		self.time.hash(state);
		self.collision.hash(state);
	}
}

#[derive(Serialize, Debug, Clone, Copy, Deserialize)]
#[wasm_bindgen]
pub struct Vector{
	pub x: f32,
	pub y: f32,
}

impl Vector {
	pub fn mag(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}

	pub fn normalized(&self) -> Vector {
		let mag = self.mag();
		Vector{
			x: self.x / mag,
			y: self.y / mag,
		}
	}

	pub fn reflect(&mut self, n: &Vector) {
		let normalized = n.normalized();
		let dot_product = self.dot(&normalized);
		self.x -= 2.0 * dot_product * normalized.x;
		self.y -= 2.0 * dot_product * normalized.y;
	}

	pub fn dot(&self, v: &Vector) -> f32 {
		self.x*v.x + self.y*v.y
	}
}

impl Hash for Vector {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.x.to_bits().hash(state);
		self.y.to_bits().hash(state);
	}
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
		let powsum = xdiff.powi(2) + ydiff.powi(2);
		let mag = G * self.mass() / powsum;
		let dist = powsum.sqrt();
		Vector{
			x: mag * xdiff / dist,
			y: mag * ydiff / dist,
		}
	}

	fn collides(&self, pos: &Vector) -> bool {
		(self.radius+PLAYER_RADIUS).powi(2) > (pos.x - self.pos.x).powi(2) + (pos.y - self.pos.y).powi(2)
	}

	fn mass(&self) -> f32 {
		self.radius*self.radius*PI
	}
}

pub fn line_intersects_circle(xp: f32, yp: f32, xc:  f32, yc: f32, rot: f32) -> bool {
	//shift everything to make line start from origin
	let a = xc - xp;
	let b = yc - yp;
	let rot_90 = rot - PI/2f32;

	//compute the quadratic's 'b' coefficient (for variable r in polar form)
	let qb = -(2f32*a*rot_90.cos() + 2f32*b*rot_90.sin());
	let discriminant: f32 = qb.powi(2) - 4f32*(a.powi(2) + b.powi(2) - PLAYER_RADIUS.powi(2));
	if discriminant < 0f32{ //no real roots (no line-circle intersection)
		return false;
	}

	let root = discriminant.sqrt();

	let r1 = (root - qb)/2f32;
	let r2 = (-root - qb)/2f32;

	let r1_good = PISTOL_REACH > r1 && r1 > 0f32;
	let r2_good = PISTOL_REACH > r2 && r2 > 0f32;

	r1_good || r2_good
}

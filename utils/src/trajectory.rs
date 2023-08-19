use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use wasm_bindgen::prelude::*;
use std::f32::consts::{PI};
use serde::{Deserialize,Serialize};
use base64::{engine::general_purpose, Engine as _};

#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
use bincode::serialize;
#[cfg(target_arch = "wasm32")]
use bincode::deserialize;

#[cfg(not(target_arch = "wasm32"))]
use rand_distr::{Normal, Distribution};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub const PLAYER_RADIUS: f32 = 25.0;
const PISTOL_REACH: f32 = 500.0; //players have circular hitbox
const DOME_RADIUS: f32 = 6000.0;
const ACCELERATION: f32 = 200.0; //player acceleration
const PROPEL_DIRECTION: f32 = -PI/2.0;
const RADIANS_PER_SECOND: f32 = PI; //player rotation speed
const G: f32 = 2000.0; //Gravitational constant

//const TIMESTEP_FPS: u32 = 10; //around 20 is good
const TIMESTEP_FPS: u32 = 30; //around 20 is good

//Calculated
const TIMESTEP_MILLIS: u32 = 1000 / TIMESTEP_FPS;
const TIMESTEP_SECS: f32 = 1f32 / TIMESTEP_FPS as f32;

#[cfg(not(target_arch = "wasm32"))]
const SPAWN_PULL_MAX: f32 = 5.0; //Maximum gravity pull at spawn point

//pub const BODIES: [Body; 1] = [
//  Body {
//    pos: Vector{
//      x: 0.0,
//      y: 0.0,
//    },  
//    radius: 100.0,
//  },  
//];

pub const BODIES: [Body; 0] = [];

#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[derive(Deserialize, Serialize)]
#[wasm_bindgen]
pub struct Trajectory{
	pub propelling: bool,
	pub pos: Vector,
	pub vel: Vector,
	pub spin: f32,
	pub spin_direction: i8, //-1,0,1
	pub time: u64,
	pub collision: bool,

	#[cfg(target_arch = "wasm32")]
	#[serde(skip)]
	updates: HashMap<u64, Vec<TrajectoryUpdate>>,
}

#[cfg(target_arch = "wasm32")]
enum TrajectoryUpdate {
	Rotation{direction: i8, hash: String},
	Propulsion{on: bool, hash: String}
}

impl Trajectory {
	#[cfg(not(target_arch = "wasm32"))]
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

	pub fn step(&mut self) -> bool{
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
			self.vel.x += -fastapprox::faster::cos(normalize_angle(self.spin + PROPEL_DIRECTION))*ACCELERATION * TIMESTEP_SECS;
			self.vel.y += fastapprox::faster::sin(normalize_angle(self.spin + PROPEL_DIRECTION))*ACCELERATION * TIMESTEP_SECS;
		}
		self.time += TIMESTEP_MILLIS as u64;
		self.collision = self.collides();
		true
	}
	
	//returns true if hash found
	#[cfg(not(target_arch = "wasm32"))]
	pub fn advance_to(&mut self, hash: String, time: u64) -> bool {
		let elapsed = (time - self.time) as u32;
		let steps = elapsed / TIMESTEP_MILLIS;
		for _ in 1..=steps {
			let current_hash = self.hash_str();
			println!("{}: {} == {} ?", self.time, current_hash, hash);
			println!("{}", self.dump());
			if current_hash == hash {
				return true;
			}
			self.step();
		}
		let current_hash = self.hash_str();
		println!("{}: {} == {} ?", self.time, current_hash, hash);
		println!("{}", self.dump());
		self.hash_str() == hash
	}

	#[cfg(not(target_arch = "wasm32"))]
	pub fn update_rotation(&mut self, new_direction: i8, hash: String, time: u64) -> bool {
		if !self.advance_to(hash, time) {
			return false;
		}
		self.spin_direction = new_direction;
		true
	}

	#[cfg(not(target_arch = "wasm32"))]
	pub fn update_propulsion(&mut self, on: bool, hash: String, time: u64) -> bool {
		if !self.advance_to(hash, time) {
			return false;
		}
		self.propelling = on;
		true
	}

	#[cfg(not(target_arch = "wasm32"))]
	fn gen_spawn() -> Vector {
		let normal = Normal::new(DOME_RADIUS/4.0, DOME_RADIUS/4.0).unwrap();
		let mut pos: Vector;
		loop {
			pos = Vector{
				x: normal.sample(&mut rand::thread_rng()),
				y: normal.sample(&mut rand::thread_rng()),
			};
			let psum = Trajectory::pull_sum(&pos);
			if psum.x.powf(2.0) + psum.y.powf(2.0) < SPAWN_PULL_MAX.powf(2.0) {
				break;
			}
		}
		pos
	}
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for Trajectory {
	fn default() -> Self {
		Trajectory {
			propelling: false,
			pos: Trajectory::gen_spawn(),
			vel: Vector{x: 0.0, y: 0.0},
			spin_direction: 0,
			spin: 0.0,
			time: current_time(),
			collision: false,
		}
	}
}

#[wasm_bindgen]
impl Trajectory {
	#[wasm_bindgen(constructor)]
	#[cfg(target_arch = "wasm32")]
	pub fn from_b64(data: String) -> Trajectory {
		let mut result: Trajectory = deserialize(&general_purpose::STANDARD.decode(&data).unwrap()).unwrap();
		result.updates = HashMap::new();
		result
	}

	//euler's method
	//returns true if change occured
	#[cfg(not(target_arch = "wasm32"))]
	pub fn advance(&mut self, time: u64) -> bool {
		if time <= self.time {
			return false;
		}
		let elapsed = (time - self.time) as u32;
		let steps = elapsed / TIMESTEP_MILLIS;
		let mut chpos = false;
		for _ in 1..=steps {
			if !self.step() {
				return chpos;
			}
			chpos = true;
		}
		true
	}

	#[cfg(target_arch = "wasm32")]
	pub fn advance(&mut self, time: u64, doprint: bool) -> bool {
		if time <= self.time {
			return false;
		}
		let elapsed = (time - self.time) as u32;
		let steps = elapsed / TIMESTEP_MILLIS;
		let mut had_update = false;
		for _ in 1..=steps {
			let hashstr = self.hash_str();
			if doprint {
				console_log!("{} ({})", self.dump(), hashstr);
			}
			let content = self.updates.get(&self.time);
			if let Some(updates) = content {
				for update in updates {
					match update {
						TrajectoryUpdate::Rotation {direction, hash} => {
							if *hash != hashstr {
								console_log!("Hash mismatch! request was {} but got {}", hash, hashstr);
							}
							self.spin_direction = *direction;
						},
						TrajectoryUpdate::Propulsion {on, hash} => {
							if *hash != hashstr {
								console_log!("Hash mismatch! request was {} but got {}", hash, hashstr);
							}
							self.propelling = *on;
						},
					}
				}
				had_update = true;
				self.updates.remove(&self.time);
			}
			self.step();
		}
		had_update
	}

	pub fn dump(&self) -> String {
		format!("{},{:x},{:x},{:x},{:x},{:x},{},{},{}", self.propelling, self.pos.x.to_bits(), self.pos.y.to_bits(), self.vel.x.to_bits(), self.vel.y.to_bits(), self.spin.to_bits(), self.spin_direction, self.time, self.collision)
	}

	#[cfg(target_arch = "wasm32")]
	pub fn set_rotation(&mut self, new_direction: i8) {
		self.spin_direction = new_direction;
	}


	#[cfg(target_arch = "wasm32")]
	pub fn set_propulsion(&mut self, on: bool) {
		self.propelling = on;
	}

	#[cfg(target_arch = "wasm32")]
	pub fn insert_rotation_update(&mut self, hash: String, spin_direction: i8, time: u64) {
		self.updates.entry(time)
		.or_default()
		.push(TrajectoryUpdate::Rotation { direction: spin_direction, hash: hash});
	}

	#[cfg(target_arch = "wasm32")]
	pub fn insert_propel_update(&mut self, hash: String, on: bool, time: u64) {
		self.updates.entry(time)
		.or_default()
		.push(TrajectoryUpdate::Propulsion { on: on, hash: hash });
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

#[cfg(not(target_arch = "wasm32"))]
pub fn current_time() -> u64 {
	let now = SystemTime::now();
	let current_time = now.duration_since(UNIX_EPOCH).expect("Broken clock");
	current_time.as_millis() as u64
}

fn normalize_angle(x: f32) -> f32 {
	let modded = ((x % (2f32*PI)) + (2f32*PI)) % (2f32*PI);
	PI - modded
}

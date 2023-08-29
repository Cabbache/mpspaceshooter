use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use wasm_bindgen::prelude::*;
use std::f32::consts::{PI};
use serde::{Deserialize,Serialize};
use base64::{engine::general_purpose, Engine as _};

#[cfg(target_arch = "wasm32")]
use std::collections::VecDeque;

#[cfg(not(target_arch = "wasm32"))]
use bincode::serialize;
#[cfg(target_arch = "wasm32")]
use bincode::deserialize;

#[cfg(not(target_arch = "wasm32"))]
use rand_distr::{Normal, Distribution};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
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

#[cfg(target_arch = "wasm32")]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

const TWOPI: f32 = 2f32*PI;
const HALFPI: f32 = PI/2f32;

pub const PLAYER_RADIUS: f32 = 25.0;
pub const DOME_RADIUS: f32 = 12000.0;

const ACCELERATION: f32 = 200.0; //player acceleration
const PROPEL_DIRECTION: f32 = -HALFPI;
const RADIANS_PER_SECOND: f32 = PI; //player rotation speed
const G: f32 = 2000.0; //Gravitational constant
const PISTOL_REACH: f32 = 500.0; //players have circular hitbox

//const REGEN: 
const TIMESTEP_FPS: u32 = 10;

//Calculated
const TIMESTEP_MILLIS: u32 = 1000 / TIMESTEP_FPS;
const TIMESTEP_SECS: f32 = 1f32 / TIMESTEP_FPS as f32;

#[cfg(not(target_arch = "wasm32"))]
pub const MAX_TIME_BEFORE: u64 = 500; //500

#[cfg(not(target_arch = "wasm32"))]
const SPAWN_PULL_MAX: f32 = 2.0; //Maximum gravity pull at spawn point
#[cfg(not(target_arch = "wasm32"))]
const MAX_TIME_AHEAD: u64 = 300; //300

pub const BODIES: [Body; 100] = [
	Body {
		pos: Vector{
			x: -3136.0,
			y: 4769.0,
		},
		radius: 180.0
	},
	Body {
		pos: Vector{
			x: -6405.0,
			y: -3068.0,
		},
		radius: 220.0
	},
	Body {
		pos: Vector{
			x: 2529.0,
			y: 2281.0,
		},
		radius: 126.0
	},
	Body {
		pos: Vector{
			x: 959.0,
			y: -8591.0,
		},
		radius: 50.0
	},
	Body {
		pos: Vector{
			x: -8826.0,
			y: -1129.0,
		},
		radius: 70.0
	},
	Body {
		pos: Vector{
			x: 8301.0,
			y: -6644.0,
		},
		radius: 189.0
	},
	Body {
		pos: Vector{
			x: -6314.0,
			y: 574.0,
		},
		radius: 226.0
	},
	Body {
		pos: Vector{
			x: -9668.0,
			y: 3578.0,
		},
		radius: 199.0
	},
	Body {
		pos: Vector{
			x: -3909.0,
			y: -2846.0,
		},
		radius: 167.0
	},
	Body {
		pos: Vector{
			x: 5546.0,
			y: 5313.0,
		},
		radius: 203.0
	},
	Body {
		pos: Vector{
			x: -7781.0,
			y: 9272.0,
		},
		radius: 191.0
	},
	Body {
		pos: Vector{
			x: 7198.0,
			y: 1724.0,
		},
		radius: 154.0
	},
	Body {
		pos: Vector{
			x: 3090.0,
			y: -6938.0,
		},
		radius: 191.0
	},
	Body {
		pos: Vector{
			x: 3469.0,
			y: 5252.0,
		},
		radius: 130.0
	},
	Body {
		pos: Vector{
			x: -3765.0,
			y: -1602.0,
		},
		radius: 126.0
	},
	Body {
		pos: Vector{
			x: -6671.0,
			y: 8253.0,
		},
		radius: 107.0
	},
	Body {
		pos: Vector{
			x: -9815.0,
			y: 4216.0,
		},
		radius: 93.0
	},
	Body {
		pos: Vector{
			x: 5313.0,
			y: -4851.0,
		},
		radius: 212.0
	},
	Body {
		pos: Vector{
			x: -7080.0,
			y: -8789.0,
		},
		radius: 159.0
	},
	Body {
		pos: Vector{
			x: 9149.0,
			y: -1318.0,
		},
		radius: 108.0
	},
	Body {
		pos: Vector{
			x: -931.0,
			y: -3439.0,
		},
		radius: 250.0
	},
	Body {
		pos: Vector{
			x: 319.0,
			y: -9557.0,
		},
		radius: 233.0
	},
	Body {
		pos: Vector{
			x: -4650.0,
			y: -6477.0,
		},
		radius: 70.0
	},
	Body {
		pos: Vector{
			x: -8701.0,
			y: 105.0,
		},
		radius: 189.0
	},
	Body {
		pos: Vector{
			x: 5664.0,
			y: 1085.0,
		},
		radius: 73.0
	},
	Body {
		pos: Vector{
			x: 645.0,
			y: -8797.0,
		},
		radius: 105.0
	},
	Body {
		pos: Vector{
			x: -9993.0,
			y: 3631.0,
		},
		radius: 82.0
	},
	Body {
		pos: Vector{
			x: 4440.0,
			y: 4571.0,
		},
		radius: 100.0
	},
	Body {
		pos: Vector{
			x: 3859.0,
			y: -5532.0,
		},
		radius: 206.0
	},
	Body {
		pos: Vector{
			x: -102.0,
			y: 9996.0,
		},
		radius: 66.0
	},
	Body {
		pos: Vector{
			x: -2296.0,
			y: -7498.0,
		},
		radius: 237.0
	},
	Body {
		pos: Vector{
			x: -9863.0,
			y: 7508.0,
		},
		radius: 234.0
	},
	Body {
		pos: Vector{
			x: -5888.0,
			y: 5060.0,
		},
		radius: 146.0
	},
	Body {
		pos: Vector{
			x: 5650.0,
			y: -7879.0,
		},
		radius: 189.0
	},
	Body {
		pos: Vector{
			x: -1139.0,
			y: -2794.0,
		},
		radius: 92.0
	},
	Body {
		pos: Vector{
			x: -2200.0,
			y: 4641.0,
		},
		radius: 212.0
	},
	Body {
		pos: Vector{
			x: -3178.0,
			y: -9673.0,
		},
		radius: 111.0
	},
	Body {
		pos: Vector{
			x: -5523.0,
			y: -7506.0,
		},
		radius: 211.0
	},
	Body {
		pos: Vector{
			x: -6394.0,
			y: 8020.0,
		},
		radius: 186.0
	},
	Body {
		pos: Vector{
			x: 7918.0,
			y: 4865.0,
		},
		radius: 131.0
	},
	Body {
		pos: Vector{
			x: 5869.0,
			y: -1728.0,
		},
		radius: 216.0
	},
	Body {
		pos: Vector{
			x: 7231.0,
			y: -7136.0,
		},
		radius: 63.0
	},
	Body {
		pos: Vector{
			x: 1230.0,
			y: -9690.0,
		},
		radius: 79.0
	},
	Body {
		pos: Vector{
			x: -9595.0,
			y: -9538.0,
		},
		radius: 96.0
	},
	Body {
		pos: Vector{
			x: -1517.0,
			y: -6934.0,
		},
		radius: 95.0
	},
	Body {
		pos: Vector{
			x: -7996.0,
			y: 5397.0,
		},
		radius: 164.0
	},
	Body {
		pos: Vector{
			x: -6981.0,
			y: -4050.0,
		},
		radius: 173.0
	},
	Body {
		pos: Vector{
			x: 3573.0,
			y: -3394.0,
		},
		radius: 240.0
	},
	Body {
		pos: Vector{
			x: -3741.0,
			y: 6714.0,
		},
		radius: 93.0
	},
	Body {
		pos: Vector{
			x: -8940.0,
			y: 8561.0,
		},
		radius: 82.0
	},
	Body {
		pos: Vector{
			x: -6630.0,
			y: 3195.0,
		},
		radius: 111.0
	},
	Body {
		pos: Vector{
			x: 9441.0,
			y: -3452.0,
		},
		radius: 222.0
	},
	Body {
		pos: Vector{
			x: -7789.0,
			y: -8423.0,
		},
		radius: 217.0
	},
	Body {
		pos: Vector{
			x: 77.0,
			y: -6083.0,
		},
		radius: 126.0
	},
	Body {
		pos: Vector{
			x: -1488.0,
			y: 9881.0,
		},
		radius: 214.0
	},
	Body {
		pos: Vector{
			x: -906.0,
			y: 9898.0,
		},
		radius: 126.0
	},
	Body {
		pos: Vector{
			x: 3787.0,
			y: -2928.0,
		},
		radius: 168.0
	},
	Body {
		pos: Vector{
			x: 9308.0,
			y: 4630.0,
		},
		radius: 77.0
	},
	Body {
		pos: Vector{
			x: -3066.0,
			y: -3291.0,
		},
		radius: 155.0
	},
	Body {
		pos: Vector{
			x: 7246.0,
			y: -8890.0,
		},
		radius: 73.0
	},
	Body {
		pos: Vector{
			x: 8570.0,
			y: 7114.0,
		},
		radius: 149.0
	},
	Body {
		pos: Vector{
			x: -3925.0,
			y: -1749.0,
		},
		radius: 196.0
	},
	Body {
		pos: Vector{
			x: 4371.0,
			y: 6218.0,
		},
		radius: 108.0
	},
	Body {
		pos: Vector{
			x: 4331.0,
			y: -4754.0,
		},
		radius: 244.0
	},
	Body {
		pos: Vector{
			x: 9945.0,
			y: -891.0,
		},
		radius: 187.0
	},
	Body {
		pos: Vector{
			x: -2557.0,
			y: 5473.0,
		},
		radius: 102.0
	},
	Body {
		pos: Vector{
			x: 9606.0,
			y: -2044.0,
		},
		radius: 82.0
	},
	Body {
		pos: Vector{
			x: -9373.0,
			y: 9178.0,
		},
		radius: 202.0
	},
	Body {
		pos: Vector{
			x: -2264.0,
			y: -198.0,
		},
		radius: 145.0
	},
	Body {
		pos: Vector{
			x: -2997.0,
			y: 6870.0,
		},
		radius: 131.0
	},
	Body {
		pos: Vector{
			x: 2920.0,
			y: -7550.0,
		},
		radius: 167.0
	},
	Body {
		pos: Vector{
			x: -4559.0,
			y: -5658.0,
		},
		radius: 187.0
	},
	Body {
		pos: Vector{
			x: 1871.0,
			y: 9080.0,
		},
		radius: 115.0
	},
	Body {
		pos: Vector{
			x: 276.0,
			y: 1672.0,
		},
		radius: 74.0
	},
	Body {
		pos: Vector{
			x: 9006.0,
			y: -6018.0,
		},
		radius: 59.0
	},
	Body {
		pos: Vector{
			x: -6771.0,
			y: 7687.0,
		},
		radius: 179.0
	},
	Body {
		pos: Vector{
			x: 5224.0,
			y: 3319.0,
		},
		radius: 69.0
	},
	Body {
		pos: Vector{
			x: -8789.0,
			y: -4801.0,
		},
		radius: 79.0
	},
	Body {
		pos: Vector{
			x: -4635.0,
			y: 6366.0,
		},
		radius: 186.0
	},
	Body {
		pos: Vector{
			x: 1087.0,
			y: 9336.0,
		},
		radius: 235.0
	},
	Body {
		pos: Vector{
			x: 3980.0,
			y: -3843.0,
		},
		radius: 183.0
	},
	Body {
		pos: Vector{
			x: -9131.0,
			y: 414.0,
		},
		radius: 187.0
	},
	Body {
		pos: Vector{
			x: 2599.0,
			y: -9358.0,
		},
		radius: 229.0
	},
	Body {
		pos: Vector{
			x: -5014.0,
			y: -1624.0,
		},
		radius: 219.0
	},
	Body {
		pos: Vector{
			x: 2518.0,
			y: 1125.0,
		},
		radius: 131.0
	},
	Body {
		pos: Vector{
			x: -7227.0,
			y: -6899.0,
		},
		radius: 53.0
	},
	Body {
		pos: Vector{
			x: 166.0,
			y: 7927.0,
		},
		radius: 203.0
	},
	Body {
		pos: Vector{
			x: -9108.0,
			y: -4844.0,
		},
		radius: 86.0
	},
	Body {
		pos: Vector{
			x: 2649.0,
			y: 7360.0,
		},
		radius: 196.0
	},
	Body {
		pos: Vector{
			x: -6710.0,
			y: -7862.0,
		},
		radius: 152.0
	},
	Body {
		pos: Vector{
			x: -9116.0,
			y: 6702.0,
		},
		radius: 222.0
	},
	Body {
		pos: Vector{
			x: 5493.0,
			y: 4757.0,
		},
		radius: 193.0
	},
	Body {
		pos: Vector{
			x: -9999.0,
			y: -1051.0,
		},
		radius: 238.0
	},
	Body {
		pos: Vector{
			x: -9154.0,
			y: -5786.0,
		},
		radius: 105.0
	},
	Body {
		pos: Vector{
			x: 1135.0,
			y: -6791.0,
		},
		radius: 142.0
	},
	Body {
		pos: Vector{
			x: -3819.0,
			y: 8999.0,
		},
		radius: 67.0
	},
	Body {
		pos: Vector{
			x: 9589.0,
			y: 8154.0,
		},
		radius: 162.0
	},
	Body {
		pos: Vector{
			x: -6915.0,
			y: -6896.0,
		},
		radius: 68.0
	},
	Body {
		pos: Vector{
			x: -9547.0,
			y: 1793.0,
		},
		radius: 116.0
	},
	Body {
		pos: Vector{
			x: -5797.0,
			y: 3215.0,
		},
		radius: 149.0
	},
];

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub struct State {
	pub x: f32,
	pub y: f32,
	pub r: f32,
}

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
	pub health: u8,
	pub boosters: u8,

	#[cfg(target_arch = "wasm32")]
	#[serde(skip)]
	updates: VecDeque<TrajectoryUpdate>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[wasm_bindgen]
pub enum UpdateType {
	RotStop,
	RotCw,
	RotCcw,
	PropOn,
	PropOff,
	AddBoost,
	Bullet,
	Health,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTypeWrapper { //because wasm-bindgen doesn't support enums with values
	pub utype: UpdateType,
	pub value: Option<u8>
}

#[wasm_bindgen]
impl UpdateTypeWrapper {
	#[wasm_bindgen(constructor)]
	pub fn new(utype: UpdateType, value: Option<u8>) -> Self {
		Self {
			utype: utype,
			value: value,
		}
	}
}

#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Debug, Clone)]
struct TrajectoryUpdate {
	time: u64,
	hash: String,
	change: UpdateTypeWrapper,
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
		if self.health == 0 {
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
		let magnitude = ACCELERATION * TIMESTEP_SECS * (self.boosters as f32);
		if self.propelling {
			self.vel.x += -fastapprox::faster::cos(normalize_angle(self.spin + PROPEL_DIRECTION)) * magnitude;
			self.vel.y += fastapprox::faster::sin(normalize_angle(self.spin + PROPEL_DIRECTION)) * magnitude;
		}
		self.time += TIMESTEP_MILLIS as u64;
		if self.collides() {
			self.health = 0;
		}
		true
	}

	#[cfg(not(target_arch = "wasm32"))]
	pub fn advance_to_time(&mut self, time: u64) {
		if time < self.time {
			return;
		}
		let elapsed = (time - self.time) as u32;
		let steps = elapsed / TIMESTEP_MILLIS;
		for _ in 1..=steps {
			self.step();
		}
	}

	#[cfg(not(target_arch = "wasm32"))]
	pub fn advance_to_time_check(&mut self, time: u64, expected_hash: String) -> bool {
		self.advance_to_time(time);
		self.hash_str() == expected_hash
	}

	#[cfg(not(target_arch = "wasm32"))]
	pub fn advance_to_min_time(&mut self, time: u64) {
		self.advance_to_time(time - MAX_TIME_BEFORE);
	}

	#[cfg(not(target_arch = "wasm32"))]
	pub fn update(&mut self, change: UpdateTypeWrapper, hash: String, update_time: u64, time: u64) -> bool {
		if update_time < self.time {
			eprintln!("Update is in the past!");
			return false;
		}
		if update_time > time && (update_time - time) > MAX_TIME_AHEAD {
			eprintln!("Update is too far ahead!");
			return false;
		}
		if update_time < time && (time - update_time) > MAX_TIME_BEFORE {
			eprintln!("Update is too long ago!");
			return false;
		}
		if !self.advance_to_time_check(update_time, hash) {
			eprintln!("Hash mismatch!");
			return false;
		}
		self.apply_change(change);
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
			if psum.x.powf(2.0) + psum.y.powf(2.0) < SPAWN_PULL_MAX.powf(2.0) && pos.mag() < DOME_RADIUS {
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
			boosters: 1,
			time: current_time(),
			health: 0xff,
		}
	}
}

#[wasm_bindgen]
impl Trajectory {
	#[wasm_bindgen(constructor)]
	#[cfg(target_arch = "wasm32")]
	pub fn from_b64(data: String) -> Trajectory {
		let mut result: Trajectory = deserialize(&general_purpose::STANDARD.decode(&data).unwrap()).unwrap();
		result.updates = VecDeque::new();
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
	pub fn advance(&mut self, time: u64) -> bool { //true if successful
		if time <= self.time {
			return true;
		}
		let elapsed = (time - self.time) as u32;
		let steps = elapsed / TIMESTEP_MILLIS;
		for _ in 1..=steps {
			loop {
				if let Some(next_update) = self.updates.front().cloned() {
					if next_update.time != self.time {
						break;
					}
					if next_update.hash != self.hash_str() {
						console_log!("Hash mismatch! request was {} but got {}", next_update.hash, self.hash_str());
						return false;
					}
					self.apply_change(next_update.change);
					self.updates.pop_front();
				} else {
					break;
				}
			}
			self.step();
		}
		true
	}

	#[cfg(target_arch = "wasm32")]
	pub fn lerp(&self, time: u64) -> State {
		if time < self.time {
			return State {
				x: self.pos.x,
				y: self.pos.y,
				r: self.spin,
			};
		}
		let delta_millis = time - self.time;
		let delta_secs = delta_millis as f32 / 1000f32;
		State {
			x: self.pos.x + self.vel.x*delta_secs,
			y: self.pos.y + self.vel.y*delta_secs,
			r: self.spin + (self.spin_direction as f32) * RADIANS_PER_SECOND * delta_secs,
		}
	}

	pub fn dump(&self) -> String {
		format!("{},{:x},{:x},{:x},{:x},{:x},{},{},{}", self.propelling, self.pos.x.to_bits(), self.pos.y.to_bits(), self.vel.x.to_bits(), self.vel.y.to_bits(), self.spin.to_bits(), self.spin_direction, self.time, self.health)
	}

	#[cfg(target_arch = "wasm32")]
	pub fn insert_update(&mut self, change: UpdateTypeWrapper, hash: String, time: u64) -> bool { //true if not in the past
		if self.time > time {
			return false;
		}
		console_log!("{:?}", change);
		self.updates.push_back(
			TrajectoryUpdate {
				time: time,
				hash: hash,
				change: change,
			}
		);
		true
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

	pub fn hits(&self, other: &Trajectory) -> f32 {
		//shift everything to make line start from origin
		let a = other.pos.x - self.pos.x;
		let b = other.pos.y - self.pos.y;
		let rot_90 = self.spin - HALFPI;

		//compute the quadratic's 'b' coefficient (for variable r in polar form)
		let qb = -(2f32*a*rot_90.cos() + 2f32*b*rot_90.sin());
		let discriminant: f32 = qb.powi(2) - 4f32*(a.powi(2) + b.powi(2) - PLAYER_RADIUS.powi(2));
		if discriminant < 0f32 { //no real roots (no line-circle intersection)
			return -1f32;
		}

		let root = discriminant.sqrt();

		let r1 = (root - qb)/2f32;
		let r2 = (-root - qb)/2f32;

		let r1_good = PISTOL_REACH > r1 && r1 > 0f32;
		let r2_good = PISTOL_REACH > r2 && r2 > 0f32;

		if !r1_good && !r2_good {
			-1f32
		} else if r1_good != r2_good {
			if r1_good {
				r1
			} else {
				r2
			}
		} else {
			f32::min(r1, r2)
		}
	}

	pub fn hash_str(&self) -> String {
		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);
		format!("{:x}", hasher.finish())
	}

	pub fn apply_change(&mut self, change: UpdateTypeWrapper){
		match change.utype {
			UpdateType::RotStop => {self.spin_direction = 0;},
			UpdateType::RotCw => {self.spin_direction = 1;},
			UpdateType::RotCcw => {self.spin_direction = -1;},
			UpdateType::PropOn => {self.propelling = true;},
			UpdateType::PropOff => {self.propelling = false;},
			UpdateType::AddBoost => {self.boosters += 1;},
			UpdateType::Bullet => {self.health = self.health.saturating_sub(change.value.unwrap_or(0));},
			UpdateType::Health => {self.health = self.health.saturating_add(change.value.unwrap_or(0));},
		}
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
		self.health.hash(state);
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

#[cfg(not(target_arch = "wasm32"))]
pub fn current_time() -> u64 {
	let now = SystemTime::now();
	let current_time = now.duration_since(UNIX_EPOCH).expect("Broken clock");
	current_time.as_millis() as u64
}

fn normalize_angle(x: f32) -> f32 {
	let modded = ((x % TWOPI) + TWOPI) % TWOPI;
	PI - modded
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn getbody(index: usize) -> Option<Body> {
	if index >= BODIES.len() {
		return None;
	}
	Some(BODIES[index].clone())
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn num_bodies() -> usize {
	BODIES.len()
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn dome_radius() -> f32 {
	DOME_RADIUS
}

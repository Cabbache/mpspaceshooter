use image::{ImageBuffer, Rgb};
use image::codecs::png::PngEncoder;
use wasm_bindgen::prelude::*;
use base64::{engine::general_purpose, Engine as _};

use rand::Rng;
use rand::rngs::SmallRng;
use rand::SeedableRng;

use kdbush::KDBush;

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

#[wasm_bindgen]
pub struct World {
	points: Vec<(f64, f64)>,
	index: KDBush,
}

#[wasm_bindgen]
impl World {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		let mut rng = SmallRng::seed_from_u64(122);
		//let points: Vec<(f64, f64)> = (1..200).map(|_| (rng.gen::<f64>() % 10000f64, rng.gen::<f64>() % 10000f64)).collect();
		let points: Vec<(f64, f64)> = (1..200).map(|_| (rng.gen::<i32>() as f64 % 1000f64, rng.gen::<i32>() as f64 % 1000f64)).collect();
		let gg = KDBush::create(points.clone(), kdbush::DEFAULT_NODE_SIZE);
		Self {
			index: gg,
			points: points,
		}
	}

	pub fn gen_slice(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> String {
		let mut result = Vec::new();
		self.index.range(x1 as f64,y1 as f64, x2 as f64, y2 as f64, |id| result.push(id));

		let w: usize = (x2 - x1).try_into().unwrap();
		let h: usize = (y2 - y1).try_into().unwrap();

		let mut raw_rgb_data: Vec<u8> = vec![0; w*h*3];
		for id in result {
			let (ptx, pty) = self.points[id];
			let (ptx, pty) = ((ptx as i32) - x1, (pty as i32) - y1);
			for a in -10..10 {
				if ptx + a < 0 || (ptx + a) as usize > w {
					continue
				}
				for b in -10..10 {
					if (pty + b) as usize > h || pty - b < 0  {
						continue
					}
					let base = (ptx+a) as usize + w*(pty+b) as usize;
					raw_rgb_data[3*base + 0] = 0xff;
					raw_rgb_data[3*base + 1] = 0xff;
					raw_rgb_data[3*base + 2] = 0xff;
				}
			}
		}
		let mut output = Vec::new();

		PngEncoder::new(&mut output)
		.encode(&raw_rgb_data, w.try_into().unwrap(), h.try_into().unwrap(), image::ColorType::Rgb8)
		.expect("Failed to encode to JPEG");

		general_purpose::STANDARD.encode(output)
	}
}

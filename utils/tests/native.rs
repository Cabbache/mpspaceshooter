#![cfg(not(target_arch = "wasm32"))]

use rand::Rng;
use utils::trajectory::Trajectory;

#[test]
fn test_some_function() {
	let mut rng = rand::thread_rng();

	const RUNS: u32 = 10000000;
	const NUM_TESTS: i32 = 50;
	for _ in 1..NUM_TESTS {
		let mut example = Trajectory::default();
		example.propelling = rng.gen::<u8>() > 128;
		example.spin = rng.gen_range(-100f32..100f32);
		example.spin_direction = rng.gen_range(-1..2);
		example.vel.x = rng.gen_range(-20f32..20f32);
		example.vel.y = rng.gen_range(-20f32..20f32);
		println!(
			"	let mut testing = Trajectory::from_b64(\"{}\".to_string());",
			example.to_b64()
		);
		println!("	for _ in 1..{} {{", RUNS);
		println!("		testing.step();");
		println!("	}}");
		for _ in 1..RUNS {
			example.step();
		}
		println!(
			"	assert_eq!(testing.hash_str(), \"{}\");",
			example.hash_str()
		);
	}
	assert_eq!(1 + 1, 2);
}

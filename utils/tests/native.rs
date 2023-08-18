#![cfg(not(target_arch = "wasm32"))]

use utils::trajectory::Trajectory;

#[test]
fn test_some_function() {
	for i in 1..50 {
		let mut example = Trajectory::default();
		example.vel.x = (i*3) as f32;
		example.vel.y = (i*-4) as f32;
		println!("	let mut testing = Trajectory::from_b64(\"{}\".to_string());", example.to_b64());
		println!("	for _ in 1..1000 {{");
		println!("		testing.step();");
		println!("	}}");
		for _ in 1..10000000 {
			example.step();
		}
		println!("	assert_eq!(testing.hash_str(), \"{}\");", example.hash_str());
	}
	assert_eq!(1+1, 2);
}

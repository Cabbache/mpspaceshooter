fn main(){
	let hex_value: u64 = 0x41d932634d84a4f2;
	let hex_value2: u32 = 0x4ec9931a;
	let decoded1: f64 = f64::from_bits(hex_value);
	let decoded2: f32 = f32::from_bits(hex_value2);
	println!("{} {}", decoded1, decoded2);
}

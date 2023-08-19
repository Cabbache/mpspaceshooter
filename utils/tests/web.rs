#![cfg(target_arch = "wasm32")]

use utils::trajectory::Trajectory;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
	let mut testing = Trajectory::from_b64("AZv9vEVcYrJDYuRCwUB9a0HoXH7BAfw/og+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5d5d31282062fd49");
	let mut testing = Trajectory::from_b64("ABwWSEWKUYlEAA0KPhR6ksEY2DRBAE5Gog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5720767167ebc342");
	let mut testing = Trajectory::from_b64("AaFkY0UKx9tEiFl2QRidR8AQugdC/5VGog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "bd8b375ddbb231fe");
	let mut testing = Trajectory::from_b64("Aa/VwER0UUlFmNnnQCtXXME4qbVBAEFHog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "6aa05fcb224d02b5");
	let mut testing = Trajectory::from_b64("AFh1H8RIgStFQKl2QdelSMFkDL1CAftMog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5b2a25c803b3cdda");
	let mut testing = Trajectory::from_b64("AM+vPkTUMk1EMFuNQPCfFkFQ5jBBAENNog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "b4321b1bc66f23b");
	let mut testing = Trajectory::from_b64("AYkQ60Qk8btEmb5AwfyIwMC41KpC/4pNog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "84216b02e5c5c23d");
	let mut testing = Trajectory::from_b64("AIzxukRHQ2NF5voHQcjDhEHSJlXCAdJNog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "3a2d3e04bc67fc26");
	let mut testing = Trajectory::from_b64("AVDKh0UAICBFZHqjwCCWXsCYRM/BABpOog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "dd6ef0db2c5494be");
	let mut testing = Trajectory::from_b64("AZInnUST56ZEAgKFwSjIz0Ap8W3CAZdTog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "2fbd01d6591ae36f");
	let mut testing = Trajectory::from_b64("AZbdUUVoZjJErLuwQCi5y0AgqJJAAeBTog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "c1318392390741b4");
	let mut testing = Trajectory::from_b64("AZkAzEQfdYhEIIhAQMUUhcHYOJhBAFBWog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "b828b939d96db1c");
	let mut testing = Trajectory::from_b64("AfBzUcS9vAxFgEnoP4DnGL/mqaXCARFcog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "d8b3ad42862b2507");
	let mut testing = Trajectory::from_b64("AGj+9ENTWKRFKth8wRoyJEG8op3CAFpcog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "cddcfafd6dca40a9");
	let mut testing = Trajectory::from_b64("ASImLUVA0vDBMcFfwbhdPEGwQLbBAKJcog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "961a75676c138d48");
	let mut testing = Trajectory::from_b64("ASTSk0TAJCnFsiGEQeA9T0E8uZFCADpiog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8794bec332d8ab91");
	let mut testing = Trajectory::from_b64("Af0FqkSt3adEwDjFwIw7U8HgLpbBADZoog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "bee50bb4d4e2dacf");
	let mut testing = Trajectory::from_b64("ASj3ZcO4PpJEwMA7QEBHG0D2q6lC/9Jtog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "811446679092c8");
	let mut testing = Trajectory::from_b64("ABb0K0V0a3ZFuNcwwLnxFcEaLQ/C/xtuog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8b8b1f401372a1de");
	let mut testing = Trajectory::from_b64("AMB6FESAu57DNNiYwVAPH0GI7KpCAWVuog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "e3fef48d63ee2889");
	let mut testing = Trajectory::from_b64("AeITLkVYMlVFaGB6wcJjVMH2rjnC/65uog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "e741a1817d39411a");
	let mut testing = Trajectory::from_b64("AWOYuEQ8FI1FoJFOQaRd+MCwuI3C//huog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "f4308104e1f86e48");
	let mut testing = Trajectory::from_b64("ATmMzkQCBflECGcaQNhESkHXv7TC/0Jvog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "119423b06956108f");
	let mut testing = Trajectory::from_b64("AKBFfEQibpVFBP9NQTpMdsFOP13CAYtvog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "1df42defd049efb8");
	let mut testing = Trajectory::from_b64("Acn6dUVhmAdFhPOMQMBgd8AiHXPCAdVvog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "c022a4ec2ce83731");
	let mut testing = Trajectory::from_b64("AXtAAESW9zRFO587wcrhkkGoWJ1BAN5wog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "d37f0a0c9bf17ff8");
	let mut testing = Trajectory::from_b64("APAAssIK1HREnBAnQX6qhkHyb6BC/8R2og+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "83a8573c15181ced");
	let mut testing = Trajectory::from_b64("AeDqD0I3F8lEPJkEwag4e8EyPrPC/w93og+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "53288318467abe40");
	let mut testing = Trajectory::from_b64("AMXBAEUEunREXHxlQYX2ksEre6/C/1l3og+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5d196991a6145dea");
	let mut testing = Trajectory::from_b64("AeplA0UJ8mZF1iqQwBCfhUF0JaFCAaV3og+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "7dd8899ef6801f43");
	let mut testing = Trajectory::from_b64("Ad/t40TgpGjC4AWwv+SLTEEwNAPBAAN4og+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "af2bceccfbf9bf78");
	let mut testing = Trajectory::from_b64("AHGGdkUEbjZEcjN6wRCZdEGZwKzCAa99og+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "6a9972ff0db34d9");
	let mut testing = Trajectory::from_b64("AfAwWMQAIEvBQE9jwdCyacD40VPBAfl9og+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "406b5b27d22287d6");
	let mut testing = Trajectory::from_b64("AcQdO0WYgpxDlAQEQdB0T8CYOAFCAEN+og+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "957d4e437579a41b");
	let mut testing = Trajectory::from_b64("AeuVF0WA3OZBLv0mQUCamD5ATOa//ziEog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "32e2633fc0aa8ffc");
	let mut testing = Trajectory::from_b64("AAqkckSmzpBEhECkQCB8hUHtU1fCAY6Eog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "93829158be605b94");
	let mut testing = Trajectory::from_b64("ATgbKkUUU89E6Kg5QQBwl0ACDhHC/9mEog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8a8dcd83fef888e0");
	let mut testing = Trajectory::from_b64("ATJ0n8TwunzDMJ16QBDdZUEgYHdC/yOFog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "742f4cf607448e12");
	let mut testing = Trajectory::from_b64("ABCzXsQMMo7DGH1vQdDkkkAMQsHCAW2Fog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "bfb760e52c213fad");
	let mut testing = Trajectory::from_b64("AHJJJETmOxVFK1+EwWQanUHACzLA/7eFog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "3aeaf161f76e8b89");
	let mut testing = Trajectory::from_b64("ALCTC0UcXllFAhufwWBWFMCyDHrCAQKGog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "15f5f4e57f6093d");
	let mut testing = Trajectory::from_b64("Adhab0O0cSNFpji3wADmET9QHBbBAUyGog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "ad6269f6c38c26ef");
	let mut testing = Trajectory::from_b64("AYszwURM87ZE+Ai6QKBGicGPF7fCAZaGog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "4d839a0397ad160e");
	let mut testing = Trajectory::from_b64("AOzRiUUuqDtEUDIrwAA8B70ARGs/AeGGog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5ba3a2261031cd20");
	let mut testing = Trajectory::from_b64("AayOkUSd8SxEpvnrwGA06kBgoMPB/yyHog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "752d05e0220a3857");
	let mut testing = Trajectory::from_b64("AH4tQkXUDBJFvWSXwUjDy0CALNE/AXaHog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "9e8964153921619d");
	let mut testing = Trajectory::from_b64("ALk7XkVeESdFMN2bQZDgE8BQMP9AAMCHog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "ae67297aad9fd83");
	let mut testing = Trajectory::from_b64("AKEwHkVH5BZFeQQlwQSb9kAYD71CAAuIog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8728d95db5ee9b06");
	let mut testing = Trajectory::from_b64("APuWcUQINQ5FXJAawXgujsGwKK3AAFeIog+KAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "623fdf77f8a980b1");
}

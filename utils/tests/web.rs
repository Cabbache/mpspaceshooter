#![cfg(target_arch = "wasm32")]

use utils::trajectory::Trajectory;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
	let mut testing = Trajectory::from_b64("AUDm20P6q9ZESGyGQTx3lcHMaRtC/7Ow5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "ed5a4deec84cac09");
	let mut testing = Trajectory::from_b64("AY5uf0UTzfpE4CBQQfmTR8GMAtHBAU605xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "aa5fc3d76d2d700f");
	let mut testing = Trajectory::from_b64("AV9ZRERTiOdE1LDjwHwZTUGEfXJCADS45xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "e1e652545f4f2cd4");
	let mut testing = Trajectory::from_b64("AF55CEUUSI1F2JIfQZjNkcGQFYpC/3a85xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "bf4e4060d54c039b");
	let mut testing = Trajectory::from_b64("AL6J7UQIzE1FHMyKwVTB30DUnK5C/0y+5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5dce2fd1364bd38a");
	let mut testing = Trajectory::from_b64("AF/qc0SdA6FF4KSiQLh9E0HatbZCAf6/5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8e629ef98e02cff0");
	let mut testing = Trajectory::from_b64("ADMCbkU1pUZFQO0VQMbjlsGwVKDCAeXB5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "ad17760c45cd26be");
	let mut testing = Trajectory::from_b64("AKQ3qsM85b9DhBXowK41gsGIC6fBAL/D5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5178aa1713247333");
	let mut testing = Trajectory::from_b64("AcxbP0XIxDdFIF1HQAAQAD+o85nCAYzF5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "cb13ab9814425164");
	let mut testing = Trajectory::from_b64("AMbFJ0TARixE0jtYwSCBecC58jPCADLK5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "ff14accb4975cc81");
	let mut testing = Trajectory::from_b64("AduNLUWgpXBEVm+KwXgVqUAgcbzCAE3M5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "83050004063d8e0c");
	let mut testing = Trajectory::from_b64("AVSRiETQEOvCSESUQURjlEHM48RCAOnQ5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5f1073567d75c3ab");
	let mut testing = Trajectory::from_b64("AdK7AkX8NEpF1MpnwSBqgj8kCl7CAaXV5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "7061643ec7eb8b9a");
	let mut testing = Trajectory::from_b64("AUR7gEPWicbEyBhyQcYQU8Ehl03C/8/Z5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "d1d060289f9bc537");
	let mut testing = Trajectory::from_b64("AfX5j0RQhDhFQBuEQPiCicC8sLvB/wze5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "e34d012dc5db5af2");
	let mut testing = Trajectory::from_b64("AcpgPkWN0MpEnOiRQRhLSkGM/ChCAWXi5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "765d03184355a94e");
	let mut testing = Trajectory::from_b64("ARGScUVIDgzEzkc3wa7/U8FcMA3CANzm5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "a285a7cca10f793e");
	let mut testing = Trajectory::from_b64("AMPdAURNqZZE2hASwdz4IUGNcx7C/yzr5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "11228b2385d427df");
	let mut testing = Trajectory::from_b64("AKBqbENYM4zDWrxywToVpsCuUnTCASnt5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "68a3edbf73fdc582");
	let mut testing = Trajectory::from_b64("ADWYA0RigRBFnn9/wXaAi8AEe3XCASvv5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "48c91d98ddae991f");
	let mut testing = Trajectory::from_b64("AOVAx0Tvnj1Eul3YwBDjKEDY+wjCAWHx5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "76a830000e92c7ce");
	let mut testing = Trajectory::from_b64("AeiuF0XHAcpEDHQPQXTdnkGwULTBAaLz5xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "4293d39429c592b1");
	let mut testing = Trajectory::from_b64("AMzIMcQRJJ9FwFfRP4f/ZsHMyMNB/+L45xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "499e6dfa11c7a35f");
	let mut testing = Trajectory::from_b64("AQCLo8CGQS5E0HsZQZAslUGmQpzCAA/75xGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "e4f466a6605b25ef");
	let mut testing = Trajectory::from_b64("ARguMkV+kQZFANmbQBBDnkHYu8DBAT8A6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "c123913f3da64c45");
	let mut testing = Trajectory::from_b64("AIaZBERy4QJFgBzEvhJXyMCA6xZAAeME6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "d501e29b8fe8dfb4");
	let mut testing = Trajectory::from_b64("AV21/URObqbE4L4Sv5DYZUEYAMDBAPUG6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "7a9064ef48af6a93");
	let mut testing = Trajectory::from_b64("AYiSQEPX6YhER+GLwXqwwMBC3ozCAQAM6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "ab369c84cc126e9d");
	let mut testing = Trajectory::from_b64("AQ44I0WhkrVEvAl8QcBYpcCActC/AMEQ6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "eeb1a007d293f520");
	let mut testing = Trajectory::from_b64("AGtgXUTojh9FAmyOweSm58BYYQnB/90U6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "daf1d2c309442581");
	let mut testing = Trajectory::from_b64("ATLJgMTQBb/DdBwnQVVBcMFHfq7CAOgW6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "703cd0ca066d577a");
	let mut testing = Trajectory::from_b64("AGz6tkQpYFtFWLmQQC9PH8E8Pv7B/3gb6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "6cfcdd5e1bdfe709");
	let mut testing = Trajectory::from_b64("AEbxA0V4unVDFMaTwM28GMF8SNTBAYAd6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "7ddf06f3b9187183");
	let mut testing = Trajectory::from_b64("AJUGNUXIRYPDiPUMQaEbGMHz87jC/3Mf6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "6c7a236c36afd5c6");
	let mut testing = Trajectory::from_b64("AMOoO0Wpb7tEpk8WQSpxmUHSe8VC/2gh6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "d120a0f6866ff40b");
	let mut testing = Trajectory::from_b64("ADucHEUkTLpD2atywfb3hMFAhgbAAGwj6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8fd5eb842b357bf3");
	let mut testing = Trajectory::from_b64("AfCO/MOAinFFkAEIQQQMG0GbiI/C/2sl6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "91ae0f2a8ebbc72f");
	let mut testing = Trajectory::from_b64("AEb1XEVSPC5FzGyUQbjVOEEk+rtC/68p6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8b2e0b4d9b9dbdc5");
	let mut testing = Trajectory::from_b64("AEC7jkV6qkFFAIu8P7hhncEAACNCAbwr6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "c926b56bc5689262");
	let mut testing = Trajectory::from_b64("AAzrMETpEKhEoGfoQHz/REF0c6RCALst6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "fd333f95cec74aff");
	let mut testing = Trajectory::from_b64("AALSGkWA6VnEuBV1QIKGP8GMzNFB/7Mv6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8180022c5fa01a4c");
	let mut testing = Trajectory::from_b64("AUQ7MUVPr3RFEFBIwLS3fUHMzzVC/68x6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "3b427c7b26258335");
	let mut testing = Trajectory::from_b64("AEWPDERIOHxDHkiUQdT3pkAn7bLC/8E16BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5571ae883f6c2921");
	let mut testing = Trajectory::from_b64("AKAXQUKoU09FVBwsQcAU8ECAv+FBAMY36BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "febd3e0e97913734");
	let mut testing = Trajectory::from_b64("AGvTNUWkjuFEMJaMQV7ujEFgEzrC/8Y56BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "f5977d381b0a9e55");
	let mut testing = Trajectory::from_b64("AVB4VkNwNRBDSGIbQQBx4EBZ1bDCAcc76BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "25b0d59b07692625");
	let mut testing = Trajectory::from_b64("AD2GD0Xlh7dEANIlPdjkTkHszK7BAT5A6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "811d86285630f080");
	let mut testing = Trajectory::from_b64("AaCKU8OqYEtFcOvXQDZD0cACwbbCADRC6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "b1b6bfe129d8303a");
	let mut testing = Trajectory::from_b64("AXx1AUSoeBhDLKNQwQ6qbME4sSJCAahG6BGKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "c609e02f8f85059e");
}

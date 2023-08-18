#![cfg(target_arch = "wasm32")]

use utils::trajectory::Trajectory;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
	let mut testing = Trajectory::from_b64("ACZCP0RuXw9FAABAQAAAgMAAAAAAABF06QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "6c532d13cd530eb7");
	let mut testing = Trajectory::from_b64("AEBSBkXWrl5EAADAQAAAAMEAAAAAAFp06QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "cd78a2c4cfe1eba4");
	let mut testing = Trajectory::from_b64("AHSHI0XLkwhFAAAQQQAAQMEAAAAAAKJ06QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "3c9643929510632f");
	let mut testing = Trajectory::from_b64("AGRwtMQLW2xFAABAQQAAgMEAAAAAAOp06QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "3b44b5f77e8fd176");
	let mut testing = Trajectory::from_b64("AER/3MPhRAhFAABwQQAAoMEAAAAAADN16QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "791092a5e7f2f484");
	let mut testing = Trajectory::from_b64("AMi9xURfY+dEAACQQQAAwMEAAAAAAHN16QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "f1a832e008b8ce2e");
	let mut testing = Trajectory::from_b64("AEZdPUXA5ChEAACoQQAA4MEAAAAAALR16QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "12b4a0a1aa20daa3");
	let mut testing = Trajectory::from_b64("AK5+1MSsrrNEAADAQQAAAMIAAAAAAPV16QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "fbc1bdb30204d081");
	let mut testing = Trajectory::from_b64("AHoO2ET2K4JFAADYQQAAEMIAAAAAADd26QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "93a3655efdd0ed96");
	let mut testing = Trajectory::from_b64("ALQMlEXxtBVFAADwQQAAIMIAAAAAAIJ56QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "e00fb4d2b15c28c3");
	let mut testing = Trajectory::from_b64("AIryf0SUpEhFAAAEQgAAMMIAAAAAABx96QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "24f3e309bd423d4b");
	let mut testing = Trajectory::from_b64("AKC+nkLuqMxEAAAQQgAAQMIAAAAAAGeA6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "88403765dde8d33a");
	let mut testing = Trajectory::from_b64("ACbST0WA4J7EAAAcQgAAUMIAAAAAAKeA6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "b43c07e04c103bf9");
	let mut testing = Trajectory::from_b64("ABwijMQjaGFFAAAoQgAAYMIAAAAAACWB6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "451607578a9be74d");
	let mut testing = Trajectory::from_b64("ANs63kTSYYFEAAA0QgAAcMIAAAAAAGeB6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "25a90bd82370280b");
	let mut testing = Trajectory::from_b64("AFxmA0VwVaJDAABAQgAAgMIAAAAAAK2E6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "b97d6ba0fb7e36b5");
	let mut testing = Trajectory::from_b64("AHoOM0UgrHpCAABMQgAAiMIAAAAAAOiH6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "49e8d4f6b6bebaab");
	let mut testing = Trajectory::from_b64("ADz5h0Mz5VJFAABYQgAAkMIAAAAAACOL6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "e933c2ad52833f0d");
	let mut testing = Trajectory::from_b64("AKpO+kTcKd5DAABkQgAAmMIAAAAAAGaO6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "9079eebbf879d7a0");
	let mut testing = Trajectory::from_b64("AELi9EP9ymNFAABwQgAAoMIAAAAAAJ+R6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "4c4bb117636938bc");
	let mut testing = Trajectory::from_b64("AFLV+kQ0fllFAAB8QgAAqMIAAAAAAOCU6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "e6cc977ba191a886");
	let mut testing = Trajectory::from_b64("AFt4Z0VkQYhEAACEQgAAsMIAAAAAACOY6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8c94053969eb44f1");
	let mut testing = Trajectory::from_b64("AHpKKEUnMgJFAACKQgAAuMIAAAAAAHab6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "924a0b13ba9d3c1e");
	let mut testing = Trajectory::from_b64("AKw1B0V6v55FAACQQgAAwMIAAAAAANye6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "294e955529e45927");
	let mut testing = Trajectory::from_b64("APOp10QVBudEAACWQgAAyMIAAAAAAB6f6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "d67ec9033571968");
	let mut testing = Trajectory::from_b64("ADx9nURukwRFAACcQgAA0MIAAAAAAI2i6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "465f9244b06460bc");
	let mut testing = Trajectory::from_b64("AFoLDcW2ZFxEAACiQgAA2MIAAAAAAPOl6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "2ff9caf2317d383");
	let mut testing = Trajectory::from_b64("AEoCD0R6dndEAACoQgAA4MIAAAAAAFup6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "d240941987cdc16f");
	let mut testing = Trajectory::from_b64("AOUjL0Vq7hlFAACuQgAA6MIAAAAAAMms6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "43616f19b80fb8c0");
	let mut testing = Trajectory::from_b64("AIhXQ0VDTgtFAAC0QgAA8MIAAAAAABGt6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "90672a4252dca5c3");
	let mut testing = Trajectory::from_b64("AIBOE0Ug2WXCAAC6QgAA+MIAAAAAAFWt6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "e99cd23aac832efd");
	let mut testing = Trajectory::from_b64("ANoem0RkTNXDAADAQgAAAMMAAAAAAMmw6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "48d9eaf807d367ca");
	let mut testing = Trajectory::from_b64("AKIG7MTciwBFAADGQgAABMMAAAAAAAyx6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "65e45dcccdd04c22");
	let mut testing = Trajectory::from_b64("AB5BZkVIRORDAADMQgAACMMAAAAAAFGx6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "a13d7629b4857196");
	let mut testing = Trajectory::from_b64("AMjJ1EOYARlFAADSQgAADMMAAAAAAJWx6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "d861b9234415020d");
	let mut testing = Trajectory::from_b64("AF4AC0W8n8RDAADYQgAAEMMAAAAAAPe06QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5ffd04eb98a9f413");
	let mut testing = Trajectory::from_b64("AE3TCkUXbxBFAADeQgAAFMMAAAAAAFu46QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "53d1112e565176e8");
	let mut testing = Trajectory::from_b64("AKOA70TwtSJFAADkQgAAGMMAAAAAAJ+46QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "c410312ce1ca4d88");
	let mut testing = Trajectory::from_b64("AD9y/kRUkj7EAADqQgAAHMMAAAAAAOW46QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "9152dfc44b2a9827");
	let mut testing = Trajectory::from_b64("AAOxbEVJJupEAADwQgAAIMMAAAAAAG286QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "cc08806f4eea9b60");
	let mut testing = Trajectory::from_b64("AJopBsU8eV9FAAD2QgAAJMMAAAAAALW86QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "263990149bbe0c46");
	let mut testing = Trajectory::from_b64("ACKqXUWvSs1EAAD8QgAAKMMAAAAAAP286QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "7ae6ad5bb503c81a");
	let mut testing = Trajectory::from_b64("AEBOFEXcEiNFAAABQwAALMMAAAAAAEW96QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "bb69467b7d519502");
	let mut testing = Trajectory::from_b64("AH5bU0VV/25FAAAEQwAAMMMAAAAAAI296QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "79121a46fa89c27b");
	let mut testing = Trajectory::from_b64("AJhIqcMLAARFAAAHQwAANMMAAAAAANa96QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "62293caacd464850");
	let mut testing = Trajectory::from_b64("AHCStUQFwgFFAAAKQwAAOMMAAAAAAB6+6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8a7ad799c15eef11");
	let mut testing = Trajectory::from_b64("ALAxJ0WizAxFAAANQwAAPMMAAAAAAGa+6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "eb9ceb25fecec7e8");
	let mut testing = Trajectory::from_b64("ANhlsERTMZpEAAAQQwAAQMMAAAAAAK6+6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "3a98406d04e02f45");
	let mut testing = Trajectory::from_b64("AEKwSkVRs5VEAAATQwAARMMAAAAAAPa+6QiKAQAAAA==".to_string());
	for _ in 1..1000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "3bcd9539e1f82660");
}

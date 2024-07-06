#![cfg(target_arch = "wasm32")]

use utils::trajectory::Trajectory;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
	let mut testing =
		Trajectory::from_b64("APuBgkQuMbJEyL+YwRhjcsCobCtB/6cQJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8b4fdd6d0a21949f");
	let mut testing =
		Trajectory::from_b64("AapOIUX+kapEfGxawXiWjEGTgqTCAf4QJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "f6921ab59f136e67");
	let mut testing =
		Trajectory::from_b64("AcQkcMQmmERFjFJfwTyjnkGoC8XBAFURJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "146b9a944698b075");
	let mut testing =
		Trajectory::from_b64("AO69isRGhwdFgOcTQMAoTUGk1DtCAewcJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "b606098113657d81");
	let mut testing =
		Trajectory::from_b64("ABmaD0XcT5REfx6ZwXiMDMFQcG/C/0QdJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "ccac7d69fa448e9a");
	let mut testing =
		Trajectory::from_b64("ACquYkWkgTVEjGl2QXwntkC8mq1CAZsdJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8f678e91cd7508a");
	let mut testing =
		Trajectory::from_b64("AfY4OEVM0E5FGOufQKhTkEB4RNpBAfMdJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "b140b5f1f75a9c3d");
	let mut testing =
		Trajectory::from_b64("AeCJKEWrlIhFC80uwfDku7+Au/o/AUoeJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "d95209bc8398b405");
	let mut testing =
		Trajectory::from_b64("AemMkkTnIAREAAm4P0oAP0EG1KnC/5seJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "574a4960760e58");
	let mut testing =
		Trajectory::from_b64("AUvHAkUQvzpFNI6GwQrtN8FMw8FC/+weJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "f8bf7ca24782a3cb");
	let mut testing =
		Trajectory::from_b64("AI+tokSa9XZEvJh2QeeFjcE476nB/0AfJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "29e4d4cf24751554");
	let mut testing =
		Trajectory::from_b64("AZGEOkSiaKlEbMNBQYjhlcDmK77CAZIfJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "a62483d1a883732a");
	let mut testing =
		Trajectory::from_b64("ANtY10QCSYLEsG5FQdCfgkAKj7fC/+UfJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "7aa84bb893c4bfe8");
	let mut testing =
		Trajectory::from_b64("AN6SFMX+DfVELiOVwXim70CrLGzC/zggJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "28fe467b6f762eb7");
	let mut testing =
		Trajectory::from_b64("AEiDwMTquHBFwMlov2pkUcHk89DB/4wgJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "27840326899689ca");
	let mut testing =
		Trajectory::from_b64("ARDnrUPMNQfEr3g5wQAJv770pSTC/+AgJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "c5c35ffc617c5bad");
	let mut testing =
		Trajectory::from_b64("AYSEtkRw0PVEqBWSwTRUOEF4oHVBATQhJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "9ab11119f9191998");
	let mut testing =
		Trajectory::from_b64("AT2mmkQuC0xE6MASQPDRBEEQfe3A/4YhJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "95e81ab3c5e3156d");
	let mut testing =
		Trajectory::from_b64("Aacg1kTiD3NFDYxUwczUmkDAp/xBAdghJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "d6993b680637f2ed");
	let mut testing =
		Trajectory::from_b64("AAj1n0OceS3EnuN/wYr2HEHysATC/ykiJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "7a09383edea6acbc");
	let mut testing =
		Trajectory::from_b64("ABGPukRMnSlFAIGRQQCq/b1Yc6dCAX4iJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "ee5e689f8f050f05");
	let mut testing =
		Trajectory::from_b64("AWCgdER8VilF+gGOwRpWgEG0fRXCAdEiJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "eae0a8fa0e8387fb");
	let mut testing =
		Trajectory::from_b64("AfvAsEXOVStEfJGDQdZEiMHQcpXA/yUjJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "7c58ab034f008a75");
	let mut testing =
		Trajectory::from_b64("AAhqSENCDWFFINRKQQKCD8H4TaZBAHojJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "4370898300ebba1f");
	let mut testing =
		Trajectory::from_b64("AP4fwsTKuwZFatiPwEZDCkF0gC3CAcsjJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "18d0df80f4bf7afa");
	let mut testing =
		Trajectory::from_b64("ADAi3EQsmEZFCBiUQQ7kQcHiNlnCAB4kJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "2b24b0b65133d882");
	let mut testing =
		Trajectory::from_b64("AVNxKUUDw3hFPK1jQeDr7j80ExRC/28kJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "6591f1957256e755");
	let mut testing =
		Trajectory::from_b64("AAViDUUN/ZxEEEVmwEh8lkFgaAlC/8EkJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "1d079a321563ac59");
	let mut testing =
		Trajectory::from_b64("AHy8kkWswnhEOIIaQJD1IcDErWhCABMlJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "8d4d3b235cb2ab1a");
	let mut testing =
		Trajectory::from_b64("AIiepUQMzWZEnCt8QXsyRcEg6BHCAWYlJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "a8027d998e1a14b8");
	let mut testing =
		Trajectory::from_b64("AaLdP0W91hxF/RwzwRIki0Hwe0FCALklJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "9442aa491e8554b7");
	let mut testing =
		Trajectory::from_b64("AYRqfURuL85EEvq7wM69DEH44x3CAcIxJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "1bc7e563dd51b761");
	let mut testing =
		Trajectory::from_b64("ARaAjkQSuvFEWjpFwaYBc8HIWT1CABQyJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "70cd778ddb1370c");
	let mut testing =
		Trajectory::from_b64("AK5JBkWaOYpFmJKsQPYkLcHcv6tCAAY+JhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "7416291b303df715");
	let mut testing =
		Trajectory::from_b64("ABybcMQShINFaOX+QJAaOcEINs5B/1s+JhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "ed915b76f0ce20ac");
	let mut testing =
		Trajectory::from_b64("AfKL+EM8k5/DTsyRwcJeQMGACRNCAK8+JhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "50766ca055472425");
	let mut testing =
		Trajectory::from_b64("ARzXiUPXM4pEuNvvQKwicEFoGnRCAKJKJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "caecd836437f39f6");
	let mut testing =
		Trajectory::from_b64("AcxqmsTsOB1FvnR5wawL8sAwPhLBASdXJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "82efb0ebc6239484");
	let mut testing =
		Trajectory::from_b64("AG4eO0UiaN1EAOWIPcwE40BEER5C/9FXJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "7f771ccdcabf3a32");
	let mut testing =
		Trajectory::from_b64("ASTplsTw+jHESriQwaD7REFgihVCAIJYJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "9686a29100af3f76");
	let mut testing =
		Trajectory::from_b64("AYJ4hMTM3nfE4EaAQIDvCkDQRmPB/4dxJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "2872e4fc7bdfd142");
	let mut testing =
		Trajectory::from_b64("ALDtRcTiHdnEbPD8QGALi0EIOMTBADlyJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5e371cd74a9a51e0");
	let mut testing =
		Trajectory::from_b64("AHikpET3uqdEkP/dwKB6fUGAO1VAAPByJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "e22560d8cbb7d6a0");
	let mut testing =
		Trajectory::from_b64("ASAfisMzIW5FYMeYP85MWsGQ7hTCAKlzJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "b7ca391aeba3fc3c");
	let mut testing =
		Trajectory::from_b64("APhKSUXxw8ZEhINzQVi2VUEsdrVC/9SLJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "686967f7cc41308c");
	let mut testing =
		Trajectory::from_b64("ANQOXkUcUQRFsMdIQGyxiUGRIIPCAIeMJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "1219940d0f979f92");
	let mut testing =
		Trajectory::from_b64("AZL5E0VhTulEJIuKwaDI9z8WpLNC/zqNJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "5e81a9cc9b90692f");
	let mut testing =
		Trajectory::from_b64("AYGZsEQYHgfFOHOuQGqdu8CU4lvCAO+NJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "bb04936f3890aeea");
	let mut testing =
		Trajectory::from_b64("AWA/OMOOnGJEwAzYPr5h0MBcHbrB/4qlJhOKAQAAAA==".to_string());
	for _ in 1..10000000 {
		testing.step();
	}
	assert_eq!(testing.hash_str(), "53fdb3b331c603b4");
}

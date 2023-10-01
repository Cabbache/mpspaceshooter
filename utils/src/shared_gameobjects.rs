use serde::Deserialize;
use wasm_bindgen::prelude::*;
use crate::trajectory::UpdateType;

#[wasm_bindgen]
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ShopItemId{
	MoreBoosters,
	Health,
	Ammunition,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ShopItem {
	pub id: ShopItemId,
	pub cost: u32,

	#[cfg(target_arch = "wasm32")]
	display_name: String,
	#[cfg(target_arch = "wasm32")]
	image_src: String,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ShopItem{
	pub fn display_name(&self) -> String {
		self.display_name.clone()
	}
	pub fn image_src(&self) -> String {
		self.image_src.clone()
	}
}

pub fn get_shop_items() -> [ShopItem; 3] {
	[
		ShopItem {
			cost: 5,
			id: ShopItemId::MoreBoosters,

			#[cfg(target_arch = "wasm32")]
			display_name: "boost".to_string(),

			#[cfg(target_arch = "wasm32")]
			image_src: "static/textures/gunshot.png".to_string(),
		},
		ShopItem {
			cost: 10,
			id: ShopItemId::Health,

			#[cfg(target_arch = "wasm32")]
			display_name: "5 health".to_string(),

			#[cfg(target_arch = "wasm32")]
			image_src: "static/textures/heart_increase.png".to_string(),
		},
		ShopItem {
			cost: 10,
			id: ShopItemId::Ammunition,

			#[cfg(target_arch = "wasm32")]
			display_name: "20 x ammo".to_string(),


			#[cfg(target_arch = "wasm32")]
			image_src: "static/textures/pistol_ammo.png".to_string(),
		},
	]
}

#[wasm_bindgen]
pub fn get_shop_item(index: usize) -> Option<ShopItem> {
	let items = get_shop_items();
	if index >= items.len() {
		return None;
	}
	Some(items[index].clone())
}

#[wasm_bindgen]
pub fn num_shop_items() -> usize {
	get_shop_items().len()
}

#[wasm_bindgen]
pub fn get_cost(utype: UpdateType) -> u32 {
	match utype{
		UpdateType::AddBoost => 5,
		_ => 0,
	}
}

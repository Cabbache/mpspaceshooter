use serde::Deserialize;
use wasm_bindgen::prelude::*;
use crate::trajectory::UpdateType;

#[wasm_bindgen]
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ShopItemId{
	MoreBoosters,
	Health,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct ShopItem {
	pub id: ShopItemId,
	pub cost: u32,

	#[cfg(target_arch = "wasm32")]
	display_name: String,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl ShopItem{
	pub fn display_name(&self) -> String {
		self.display_name.clone()
	}
}

pub fn get_shop_items() -> [ShopItem; 1] {
	[
		ShopItem {
			cost: 5,
			id: ShopItemId::MoreBoosters,

			#[cfg(target_arch = "wasm32")]
			display_name: "boosters".to_string(),
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

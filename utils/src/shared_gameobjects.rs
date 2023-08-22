use serde::Deserialize;
use wasm_bindgen::prelude::*;

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

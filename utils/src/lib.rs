pub mod shared_gameobjects;
pub mod trajectory;

#[cfg(target_arch = "wasm32")]
pub mod background;

#[cfg(not(target_arch = "wasm32"))]
pub mod server_gameobjects;

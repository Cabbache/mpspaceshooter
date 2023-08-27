pub mod trajectory;
pub mod shared_gameobjects;

#[cfg(target_arch = "wasm32")]
pub mod background;

#[cfg(not(target_arch = "wasm32"))]
pub mod server_gameobjects;

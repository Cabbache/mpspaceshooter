pub mod trajectory;
pub mod shared_gameobjects;

#[cfg(not(target_arch = "wasm32"))]
pub mod server_gameobjects;

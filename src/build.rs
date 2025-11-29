fn main() {
    // UniFFI scaffolding only for native platforms, not WASM
    // WASM uses wasm-bindgen directly
    #[cfg(not(target_arch = "wasm32"))]
    {
        uniffi::generate_scaffolding("src/my_crate.udl").unwrap();
    }
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    slint_build::compile("ui/main.slint").unwrap();
}

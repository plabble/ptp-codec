fn main() {
    #[cfg(all(feature = "unicli", not(test)))]
    uniffi::uniffi_bindgen_main();
}

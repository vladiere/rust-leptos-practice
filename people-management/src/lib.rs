pub mod app;
pub mod services;
use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

    use crate::services::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(App);
    }
}
}

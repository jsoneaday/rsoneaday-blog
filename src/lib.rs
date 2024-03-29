pub mod app;
pub mod layout;
pub mod pages {
  pub mod home;
  pub mod admin;
  pub mod post;
  pub mod not_found;
  pub mod error_boundary;
}

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(App);
    }
}
}

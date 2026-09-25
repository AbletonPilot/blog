pub mod app;
pub mod components;
pub mod performance;
pub mod posts;
pub mod rss;
pub mod sitemap;

pub const SITE_URL: &str = match option_env!("SITE_URL") {
  Some(value) => value,
  None => "https://abletonpilot.onrender.com",
};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
  use crate::app::*;
  console_error_panic_hook::set_once();
  leptos::mount::hydrate_body(App);
}

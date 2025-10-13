use leptos::prelude::*;

#[component]
pub fn Giscus() -> impl IntoView {
  // Load Giscus script on component mount
  Effect::new(move |_| {
    #[cfg(target_arch = "wasm32")]
    {
      use wasm_bindgen::JsCast;

      let window = web_sys::window().expect("no global `window` exists");
      let document = window.document().expect("should have a document on window");

      // Remove existing Giscus script and widget if present
      if let Some(existing_script) = document
        .query_selector("script[src*='giscus.app']")
        .ok()
        .flatten()
      {
        existing_script.remove();
      }
      if let Some(existing_widget) = document.query_selector(".giscus").ok().flatten() {
        existing_widget.remove();
      }

      // Create new Giscus script
      if let Ok(script) = document.create_element("script") {
        let script = script.dyn_into::<web_sys::HtmlScriptElement>().unwrap();

        script.set_src("https://giscus.app/client.js");
        script.set_attribute("data-repo", "AbletonPilot/blog").ok();
        script.set_attribute("data-repo-id", "YOUR_REPO_ID").ok(); // Will be updated after GitHub setup
        script.set_attribute("data-category", "Announcements").ok();
        script
          .set_attribute("data-category-id", "YOUR_CATEGORY_ID")
          .ok(); // Will be updated after GitHub setup
        script.set_attribute("data-mapping", "pathname").ok();
        script.set_attribute("data-strict", "0").ok();
        script.set_attribute("data-reactions-enabled", "1").ok();
        script.set_attribute("data-emit-metadata", "0").ok();
        script.set_attribute("data-input-position", "bottom").ok();
        script
          .set_attribute("data-theme", "preferred_color_scheme")
          .ok();
        script.set_attribute("data-lang", "ko").ok();
        script.set_attribute("crossorigin", "anonymous").ok();
        script.set_async(true);

        if let Some(giscus_container) = document.get_element_by_id("giscus-container") {
          giscus_container.append_child(&script).ok();
        }
      }
    }
  });

  view! {
    <div class="giscus-wrapper">
      <div id="giscus-container"></div>
    </div>
  }
}

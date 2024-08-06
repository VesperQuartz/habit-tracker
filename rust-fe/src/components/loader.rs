use dioxus::prelude::*;

pub fn Loader() -> Element {
  rsx! {
    span { class: "block w-5 h-5 rounded-full animate-spin border-2 border-white border-b-transparent" }
  }
}

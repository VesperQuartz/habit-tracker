use dioxus::prelude::*;

use crate::asset::S1;
use crate::components::accordion::Accordion;
pub fn DailyHabit() -> Element {
  rsx! {
    main {
      div {
        p { class: "text-2xl p-3 font-bold", "Daily habit" }
      }
      div { class: "flex justify-between",
        div { class: "grow flex-1",
          Accordion { id: "123456", text: "Make breakfast" }
        }
        div {
          img { src: "{S1}" }
        }
      }
    }
  }
}

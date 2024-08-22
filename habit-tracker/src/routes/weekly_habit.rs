use dioxus::prelude::*;

pub fn WeeklyHabit() -> Element {
  rsx! {
    main {
      div {
        p { "Weekly habit" }
      }
      div {
        div {}
        div {}
      }
    }
  }
}

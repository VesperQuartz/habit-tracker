use dioxus::prelude::*;

use crate::{
  components::{navbar::NavBar, sidebar::SideBar},
  Route,
};

#[derive(Clone, Copy)]
pub struct ShowSide(pub bool);

pub fn RootLayout() -> Element {
  use_context_provider(|| Signal::new(ShowSide(false)));
  let show_side = use_context::<Signal<ShowSide>>();
  rsx! {
    div { class: "flex",
      div { class: "flex",
        if show_side().0 {
          SideBar {}
        }
      }
      div { class: "flex flex-1 grow",
        div { class: "flex flex-col flex-1 grow",
          NavBar {}
          Outlet::<Route> {}
        }
      }
    }
  }
}

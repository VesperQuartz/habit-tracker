#![allow(non_snake_case)]
pub mod components;
pub mod routes;
use components::navbar::NavBar;
use dioxus::{launch, prelude::*};
use routes::login::Login;
use routes::signup::Register;

#[derive(Routable, PartialEq, Clone, Debug)]
#[rustfmt::skip]
enum Route {
	#[layout(NavBar)]
		#[route("/")]
		Home {},
	#[end_layout]
  #[route("/login")]
  Login {},
  #[route("/signup")]
  Register {},
  #[route("/:..segments")]
  NotFound { segments: Vec<String> },
}

fn Home() -> Element {
  rsx!(main {})
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
  rsx!(div {})
}

fn main() {
  launch(|| {
    rsx! {
      Router::<Route> {}
    }
  });
}

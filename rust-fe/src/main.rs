#![allow(non_snake_case)]
pub mod components;
pub mod routes;
pub mod services;
pub mod types;
use components::navbar::NavBar;
use dioxus::{launch, prelude::*};
use dioxus_logger::tracing::Level;
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
  dioxus_logger::init(Level::INFO).expect("logger failed to init");
  launch(|| {
    rsx! {
      Router::<Route> {}
    }
  });
}

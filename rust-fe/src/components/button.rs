use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
  text: String,
  onclick: EventHandler<MouseEvent>,
}

pub fn Button(props: ButtonProps) -> Element {
  rsx! {
    button {class:"font-bold h-10 w-40 p-3 m-3 text-2xl", onclick: move |event| props.onclick.call(event) , "{props.text}"}
  }
}

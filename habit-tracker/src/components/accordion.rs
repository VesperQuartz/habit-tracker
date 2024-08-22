use dioxus::prelude::*;
use dioxus_logger::tracing::info;

#[derive(PartialEq, Clone, Props, Debug)]
pub struct AccordionProps {
  text: String,
  id: String,
}

pub fn Accordion(props: AccordionProps) -> Element {
  let mut show = use_signal(|| false);
  info!("{:?}", props);
  rsx! {
    div { class: "p-3",
      div { class: "flex justify-between items-center p-2 drop-shadow-md bg-[#F9F9F9] h-[3.37rem]",
        p { class: "text-[#333333] font-bold text-xl", "{props.text}" }
        if show() {
          img { src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiMxQTM2MzYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS1taW51cyI+PHBhdGggZD0iTTUgMTJoMTQiLz48L3N2Zz4=" }
        } else {
          img { src: "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IiMxQTM2MzYiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBjbGFzcz0ibHVjaWRlIGx1Y2lkZS1wbHVzIj48cGF0aCBkPSJNNSAxMmgxNCIvPjxwYXRoIGQ9Ik0xMiA1djE0Ii8+PC9zdmc+" }
        }
      }
      div {}
    }
  }
}

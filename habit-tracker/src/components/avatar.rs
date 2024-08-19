use dioxus::prelude::*;

pub fn Avatar() -> Element {
  rsx! {
    div {
      div { class: "flex items-center gap-2 bg-white w-[6.25rem] rounded-full",
        img {
          class: "rounded-full w-[6.25rem] h-[6.25rem]",
          src: "https://img.freepik.com/free-psd/3d-illustration-person-with-sunglasses_23-2149436188.jpg?w=1380&t=st=1724033240~exp=1724033840~hmac=e253bed392c7e7409bcad6338ad93fad4fd67ae26a63614490400776655698ca",
          alt: "avatar",
        }
      }
    }
  }
}

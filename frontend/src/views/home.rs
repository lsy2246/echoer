use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { "hello,world" }
    }
}

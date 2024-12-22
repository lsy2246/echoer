use dioxus::prelude::*;
use crate::components::Message;

#[component]
pub fn Home() -> Element {
    rsx! {
        Message{},
        div {
            class:"bg-yellow-200 dark:bg-sky-500",
            "hello,world"
            ul {
                li { "nihao" }
             }
            }
    }
}

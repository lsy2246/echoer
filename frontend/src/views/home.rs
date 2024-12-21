use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class:"bg-yellow-200 dark:bg-sky-500",
            "hello,world"
            ul {
                li { "nihao" }
             }
            }
    }
}

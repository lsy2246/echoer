use crate::components::Toast;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class:"border-2 border-red-500 h-[999px]",
            "hello,world"
            ul {
                li { "nihao" }
            }
            button {
                onclick: move |_| {
                    Toast::info("我是标sasadc我想二次TV要不以后牛魔题".to_string(), "我是内容".to_string())
                },
                "info "
            }
            button {
                onclick: move |_| {
                    Toast::error("我是标sasadc我想二次TV要不以后牛魔题".to_string(), "我是内容".to_string())
                },
                "error "
            }
            button {
                onclick: move |_| {
                    Toast::success("我是标sasadc我想二次TV要不以后牛魔题".to_string(), "我是内容".to_string())
                },
                "success"
            }


        }
    }
}

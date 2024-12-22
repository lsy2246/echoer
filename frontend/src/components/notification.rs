use crate::common::error::{CustomErrorInto, CustomResult};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::bs_icons::BsXLg;

#[derive(PartialEq, Clone)]
pub enum MessageType {
    Info,
    Warn,
    Error,
    Success,
}

#[derive(PartialEq, Props, Clone)]
pub struct MessageProps {
    #[props(default="wight".to_string())]
    color: String,
    #[props(default="".to_string())]
    title: String,
    #[props(default="".to_string())]
    message: String,
    #[props(default=3)]
    time:u64,
}

#[component]
pub fn Message(props: MessageProps) -> Element {
    return rsx! {
        div {
            class: "w-[20rem] px-2 py-3 absolute right-8 top-10  backdrop-blur-xl  max-sm:w-[12rem] hover:border-2 border-accent  rounded ",
            style: "background-color:{props.color}",
            
            div { 
                
                class:"text-lg",
                div { 
                    class:"float-end relative top-1 right-1",
                    Icon{
                        icon:BsXLg,
                    }
                 }
                 div { 
                    class:"truncate",
                    "我是标题水水水水水水水水水水水水水水水水水水水水水sssssssssssss水水水ssssssssssssssss"

                  }
             }
            div {
                class:"text-base",
                "我是水水水水水水水水水ssssssssssssssssssssss水水水水内容"
            }
            
          }
    };
}

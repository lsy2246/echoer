use crate::common::dom::{set_element_class, set_local_storage_value};
use dioxus::{logger::tracing, prelude::*};
use dioxus_free_icons::icons::bs_icons::BsMoonStars;
use dioxus_free_icons::icons::bs_icons::BsSun;
use dioxus_free_icons::Icon;
#[derive(Debug, Clone, PartialEq)]
pub struct IsDark(pub String);

#[derive(PartialEq, Props, Clone)]
pub struct ToggleProps {
    #[props(default = 20)]
    pub height: u32,
    #[props(default = 20)]
    pub width: u32,
    #[props(default = "currentColor".to_string())]
    pub fill: String,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let mut dark_context = use_context::<Signal<IsDark>>();

    rsx! {
        div {
            onclick: move |_|
            {
                let theme;
                if dark_context().0=="light" {
                    theme="dark"
                }else {
                    theme="light"
                };

                dark_context.set(IsDark(theme.to_string()));
                match set_local_storage_value("theme",theme) {
                    Ok(_)=>{},
                    Err(_)=>{
                        tracing::error!("主题储存失败");
                    },
                }
                match set_element_class("html",theme) {
                    Ok(_)=>{},
                    Err(e)=>{
                        tracing::error!("主题类名设置失败:{}",e);
                    },

                }
            }
             ,
            {
            match {dark_context().0.as_str()} {
                "dark"=>{
                    rsx!(
                        Icon{
                            width:props.width,
                            height:props.height,
                            fill:props.fill,
                            icon:BsMoonStars,
                        }
                    )
                },
                _=>{
                    rsx!{
                        Icon{
                            width:props.width,
                            height:props.height,
                            fill:props.fill,
                            icon:BsSun,
                        }
                    }

                }
            }

        }

         }

    }
}

use dioxus::{logger::tracing, prelude::*};
use dioxus_free_icons::icons::bs_icons::BsMoonStars;
use dioxus_free_icons::icons::bs_icons::BsSun;
use dioxus_free_icons::Icon;
use crate::{IsDark,set_local_storage_value};

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
    let mut dark_context=use_context::<Signal<IsDark>>();

    rsx! {
        div {
            onclick: move |_| 
            {
                dark_context.set(IsDark(!dark_context().0));
                match set_local_storage_value("theme", if !dark_context().0 { "true" } else { "false" }) {
                    Ok(_)=>{},
                    Err(_)=>{
                        tracing::error!("主题储存失败");
                    },
                }
            }
             ,
            {
            match {dark_context().0} {
                false=>{
                    rsx!(
                        Icon{
                            width:props.width,
                            height:props.height,
                            fill:props.fill,
                            icon:BsMoonStars,
                        }
                    )
                },
                true=>{
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

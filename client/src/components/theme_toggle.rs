use crate::utils::dom::{
    add_element_class, get_local_storage_value, get_media_theme, remove_element_class,
    remove_local_storage_value, set_local_storage_value,
};
use dioxus::{logger::tracing, prelude::*};
use dioxus_free_icons::icons::bs_icons::BsMoonStars;
use dioxus_free_icons::icons::bs_icons::BsSun;
use dioxus_free_icons::Icon;

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeProvider(pub String);

#[derive(PartialEq, Props, Clone)]
pub struct ToggleProps {
    #[props(default = 20)]
    pub height: u32,
    #[props(default = 20)]
    pub width: u32,
    #[props(default = "currentColor")]
    pub fill: &'static str,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let mut theme_context = use_context::<Signal<ThemeProvider>>();

    rsx! {
        div {
            style: "height: {props.height}px;width: {props.width}px;",
            class:"inline-flex items-center justify-center",
            onclick: move |_|
            {
                let system_theme=get_media_theme().unwrap_or_else(|_|"".to_string());
                let target_theme;

                if theme_context().0=="light" {
                    target_theme="dark".to_string()
                }else {
                    target_theme="light".to_string()
                };

                let _=remove_element_class("html", &theme_context().0);
                theme_context.set(ThemeProvider(target_theme.clone()));
                let _=add_element_class("html", &target_theme);
                if target_theme==system_theme {
                    let _=remove_local_storage_value("theme");
                }else{
                    let _=set_local_storage_value("theme", &target_theme);
                }

            }
             ,
            {
            match {theme_context().0.as_str()} {
                "dark"=>{
                    rsx!(
                        Icon{
                            height:props.height/3*2,
                            width:props.width/3*2,
                            fill:props.fill,
                            icon:BsMoonStars,
                        }
                    )
                },
                _=>{
                    rsx!{
                        Icon{
                            height:props.height/3*2,
                            width:props.width/3*2,
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

pub fn get_theme() -> String {
    let storage_theme = get_local_storage_value("theme");
    match storage_theme {
        Ok(s) => s,
        Err(_) => {
            let device_theme = get_media_theme();
            match device_theme {
                Ok(s) => s,
                Err(_) => "light".to_string(),
            }
        }
    }
}

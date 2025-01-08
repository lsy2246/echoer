use super::Toggle;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { 
            class: "border-red-500 bg-slate-400 mb-5",
            div { 
                class: "grid grid-cols-3 items-center text-center w-[80%] mx-auto h-12",
                div {
                    class:"justify-self-start text-xl",
                    Link {
                        to: Route::Home {},
                        "echoer"
                    }
                }
                div {
                    class:"text-xl flex gap-10 justify-center",
                    Link {
                        to: Route::Home {},
                        "首页"
                    }
                    Link {
                        to: Route::Home {},
                        "首页"
                    }
                    Link {
                        to: Route::Home {},
                        "首页"
                    }
                }
                
                div {
                    class:"justify-self-end",
                    Toggle {
                        height: 35,
                        width: 35
                    }
                }
            }
        }
        div {
            class:"w-[80%] mx-auto",
            Outlet::<Route> {}
        }
    }
}

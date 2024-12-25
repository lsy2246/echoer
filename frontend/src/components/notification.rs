use crate::common::error::{CustomErrorInto, CustomResult};
use crate::common::helps::generate_random_string;
use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{BsCheckCircle, BsInfoCircle, BsXCircle, BsXLg};
use dioxus_free_icons::Icon;

#[derive(PartialEq, Clone)]
pub struct NotificationProvider {
    pub notifications: Vec<NotificationProps>,
}

#[derive(PartialEq, Clone)]
pub enum NotificationType {
    Info,
    Error,
    Success,
}

#[derive(PartialEq, Props, Clone)]
pub struct NotificationProps {
    #[props(default="".to_string())]
    title: String,
    #[props(default="".to_string())]
    content: String,
    #[props(default = 5)]
    time: u64,
    #[props(default=NotificationType::Info)]
    r#type: NotificationType,
}

#[derive(PartialEq, Clone)]
struct NoticationColorMatching {
    color: String,
    icon: Element,
}

fn get_color_matching(notification_type: &NotificationType) -> NoticationColorMatching {
    match notification_type {
        NotificationType::Info => NoticationColorMatching {
            color: String::from("rgba(0,168,91,0.85)"),
            icon: rsx! {
                Icon {
                    icon: BsInfoCircle,
                    width:18,
                    height:18,
                }
            },
        },
        NotificationType::Error => NoticationColorMatching {
            color: String::from("rgba(225,45,57,0.85)"),
            icon: rsx! {
                Icon {
                    icon: BsXCircle,
                    width:18,
                    height:18,
                }
            },
        },
        NotificationType::Success => NoticationColorMatching {
            color: String::from("rgba(38,131,255,0.85)"),
            icon: rsx! {
                Icon {
                    icon: BsCheckCircle,
                    width:18,
                    height:18,
                }
            },
        },
    }
}

#[component]
pub fn Notification() -> Element {
    let notifications = use_context::<Signal<NotificationProvider>>()().notifications;
    return rsx! {
        div {
            class: "w-[20rem] absolute right-8 top-10 max-sm:w-[12rem]",
            {notifications.iter().map(|item| {
                let color_matching=get_color_matching(&item.r#type);
                rsx!{
                    div {
                        id:format!("notification-{}",generate_random_string(10)),
                        class:"rounded px-3 py-3 m-2 !border-none text-gray-300 dark:text-gray-800 ",
                        style:format!("background-color:{}",color_matching.color),
                        div {
                            div {
                                class:"flex items-center justify-between",
                                div {
                                    onclick:|e|{

                                    },
                                    class:"mr-2 relative top-[0.06rem] hover:text-accent",
                                    {color_matching.icon},
                                }
                                div {
                                    class:"truncate min-w-0",
                                    {item.title.clone()}
                                }
                                div {
                                    class:"ml-1 flex-shrink-0",
                                    Icon{
                                        icon:BsXLg,
                                        height:18,
                                        width:18
                                    }
                                }
                            }
                            div { 
                                class:"mt-2 h-1 bg-white animate-progress",
                                style: format!("animation-duration: {}s", item.time),
                             }
                        }
                    }
                }
            })}
         }
        Outlet::<Route> {}
    };
}

pub struct Toast();

impl Toast {
    pub fn show(props: NotificationProps) {
        let mut notification_context = use_context::<Signal<NotificationProvider>>();
        let mut new_provider = notification_context().clone();
        new_provider.notifications.push(props);
        notification_context.set(new_provider);
    }
    pub fn info(title: String, content: String) {
        Self::show(NotificationProps {
            title,
            content,
            time: 5,
            r#type: NotificationType::Info,
        });
    }
    pub fn error(title: String, content: String) {
        Self::show(NotificationProps {
            title,
            content,
            time: 5,
            r#type: NotificationType::Error,
        });
    }
    pub fn success(title: String, content: String) {
        Self::show(NotificationProps {
            title,
            content,
            time: 5,
            r#type: NotificationType::Success,
        });
    }
}

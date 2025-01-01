use crate::common::error::{CustomErrorInto, CustomResult};
use crate::common::helps::generate_random_string;
use crate::Route;
use dioxus::{logger::tracing, prelude::*};
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
    id: String,
    title: String,
    content: String,
    time: u64,
    r#type: NotificationType,
}

impl Default for NotificationProps {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            title: "".to_string(),
            content: "".to_string(),
            time: 5,
            r#type: NotificationType::Info,
        }
    }
}

#[derive(PartialEq, Clone)]
struct NoticationColorMatching {
    color: String,
    icon: Element,
}

fn get_color_matching(notification_type: &NotificationType) -> NoticationColorMatching {
    match notification_type {
        NotificationType::Info => NoticationColorMatching {
            color: String::from("rgb(38,131,255)"),
            icon: rsx! {
                Icon {
                    icon: BsInfoCircle,
                    width:18,
                    height:18,
                }
            },
        },
        NotificationType::Error => NoticationColorMatching {
            color: String::from("rgb(225,45,57)"),
            icon: rsx! {
                Icon {
                    icon: BsXCircle,
                    width:18,
                    height:18,
                }
            },
        },
        NotificationType::Success => NoticationColorMatching {
            color: String::from("rgb(0,168,91)"),
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

pub fn remove_notification(id: String) {
    let mut notifications_context = use_context::<Signal<NotificationProvider>>();
    let new_notifications:Vec<NotificationProps> = notifications_context.clone()()
        .notifications
        .into_iter()
        .filter(|i| i.id != id )
        .collect();

    notifications_context.set(NotificationProvider {
        notifications: new_notifications,
    });
}

#[component]
pub fn Notification() -> Element {
    let notifications_context = use_context::<Signal<NotificationProvider>>();
    let notifications = notifications_context().clone();

    return rsx! {
        div {
            class: "w-[20rem] absolute right-8 top-10 max-sm:w-[12rem]",
            {notifications.notifications.iter().map(move |item| {
                let color_matching=get_color_matching(&item.r#type);
                let id = item.id.clone();
                rsx!{
                    div {
                        id: id.clone(),
                        class:"rounded px-3 py-3 m-2 !border-none text-gray-800 dark:text-gray-200 opacity-75 ",
                        style:format!("background-color:{}",color_matching.color),
                        div {
                            div {
                                class:"flex items-center justify-between",
                                div {
                                    class:"mr-2 relative top-[0.06rem] hover:text-accent",
                                    {color_matching.icon},
                                }
                                div {
                                    class:"truncate min-w-0",
                                    {item.title.clone()}
                                }
                                div {
                                    onclick:move |e|{
                                        e.prevent_default();
                                        remove_notification(id.clone());
                                    },
                                    class:"ml-1 flex-shrink-0",
                                    Icon{
                                        icon:BsXLg,
                                        height:18,
                                        width:18
                                    }
                                }
                            }
                            div {
                                class:"mt-2 h-1 relative",
                                div {
                                    class:"absolute bg-slate-400 w-full h-full",
                                }
                                div { 
                                    class:"absolute w-full h-full ",
                                    style:"width:80%"
                                 }
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
        let mut new_props = props;
        new_props.id = format!("notification-{}",generate_random_string(10));
        new_provider.notifications.push(new_props);
        notification_context.set(new_provider);
    }
    pub fn info(title: String, content: String) {
        Self::show(NotificationProps {
            id: Default::default(),
            title,
            content,
            time: Default::default(),
            r#type: NotificationType::Info,
        });
    }
    pub fn error(title: String, content: String) {
        Self::show(NotificationProps {
            id: Default::default(),
            title,
            content,
            time: Default::default(),
            r#type: NotificationType::Error,
        });
    }
    pub fn success(title: String, content: String) {
        Self::show(NotificationProps {
            id: Default::default(),
            title,
            content,
            time: Default::default(),
            r#type: NotificationType::Success,
        });
    }
}

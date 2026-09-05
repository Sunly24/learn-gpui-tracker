use gpui_kit::*;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::component::{Icon, IconName};

pub fn sidebar() -> impl IntoElement {
    div()
        .w(px(232.0))
        .h_full()
        .flex()
        .flex_col()
        .bg(rgb(0x131416))
        .border_r_1()
        .border_color(rgb(0x272a2f))
        .px(px(10.0))
        .py(px(10.0))
        .gap(px(14.0))
        // Workspace switcher
        .child(
            div()
                .h(px(34.0))
                .flex()
                .items_center()
                .px(px(7.0))
                .rounded(px(7.0))
                .hover(|this| this.bg(rgb(0x1d2024)))
                .child(img("org.svg").size(px(22.0)).rounded(px(6.0)))
                .child(
                    div()
                        .ml(px(9.0))
                        .flex_1()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::MEDIUM)
                        .child("SMBL"),
                )
                .child(
                    Icon::new(IconName::ChevronDown)
                        .size(px(14.0))
                        .text_color(rgb(0x777b83)),
                ),
        )
        // Search bar
        .child(
            div()
                .h(px(32.0))
                .flex()
                .items_center()
                .px(px(8.0))
                .rounded(px(7.0))
                .bg(rgb(0x181a1d))
                .border_1()
                .border_color(rgb(0x272a2f))
                .child(
                    Icon::new(IconName::Search)
                        .size(px(14.0))
                        .text_color(rgb(0x777b83)),
                )
                .child(
                    div()
                        .ml(px(8.0))
                        .flex_1()
                        .text_color(rgb(0x777b83))
                        .text_size(px(12.0))
                        .child("Search"),
                )
                .child(
                    div()
                        .px(px(5.0))
                        .py(px(2.0))
                        .rounded(px(4.0))
                        .border_1()
                        .border_color(rgb(0x303339))
                        .text_color(rgb(0x696d75))
                        .text_size(px(10.0))
                        .child("⌘K"),
                ),
        )
        // Nav items
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(1.0))
                .child(sidebar_item(IconName::Inbox, "Inbox", Some("3"), false))
                .child(sidebar_item(IconName::User, "My issues", Some("7"), false))
                .child(sidebar_item(IconName::CircleX, "Active issues", None, true))
                .child(sidebar_item(IconName::FolderOpen, "Projects", None, false))
                .child(sidebar_item(IconName::PanelLeft, "Views", None, false)),
        )
        // Projects section
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.0))
                .child(
                    div()
                        .px(px(8.0))
                        .pt(px(4.0))
                        .pb(px(4.0))
                        .text_size(px(10.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x656970))
                        .child("PROJECTS"),
                )
                .child(project_item("Website", 0x7c6ff2))
                .child(project_item("Mobile", 0x4e9f7a))
                .child(project_item("API", 0xd9a441)),
        )
}

fn sidebar_item(
    icon: IconName,
    label: &'static str,
    count: Option<&'static str>,
    active: bool,
) -> impl IntoElement {
    div()
        .h(px(31.0))
        .flex()
        .items_center()
        .px(px(8.0))
        .rounded(px(6.0))
        .when(active, |this| this.bg(rgb(0x202227)))
        .hover(|this| this.bg(rgb(0x1d2024)))
        .child(Icon::new(icon).size(px(14.0)).text_color(if active {
            rgb(0xd9dbe0)
        } else {
            rgb(0x777b83)
        }))
        .child(
            div()
                .ml(px(9.0))
                .flex_1()
                .text_size(px(12.0))
                .text_color(if active { rgb(0xe0e1e4) } else { rgb(0x858990) })
                .child(label),
        )
        .when_some(count, |this, count| {
            this.child(
                div()
                    .text_size(px(10.0))
                    .text_color(rgb(0x656970))
                    .child(count),
            )
        })
}

fn project_item(label: &'static str, color: u32) -> impl IntoElement {
    div()
        .h(px(29.0))
        .flex()
        .items_center()
        .px(px(8.0))
        .rounded(px(6.0))
        .hover(|this| this.bg(rgb(0x1d2024)))
        .child(div().size(px(8.0)).rounded_full().bg(rgb(color)))
        .child(
            div()
                .ml(px(10.0))
                .text_size(px(12.0))
                .text_color(rgb(0x858990))
                .child(label),
        )
}

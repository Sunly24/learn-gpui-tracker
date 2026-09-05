use gpui_kit::*;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::component::{Sizable, button::Button, button::ButtonVariants};

pub fn main_content() -> impl IntoElement {
    div()
        .flex_1()
        .h_full()
        .flex()
        .flex_col()
        // Top breadcrumb bar
        .child(
            div()
                .h(px(56.0))
                .flex()
                .items_center()
                .px(px(28.0))
                .border_b_1()
                .border_color(rgb(0x272a2f))
                .child(
                    div()
                        .text_color(rgb(0x686c74))
                        .text_size(px(12.0))
                        .child("Issues"),
                )
                .child(div().mx(px(8.0)).text_color(rgb(0x44474d)).child("/"))
                .child(div().text_size(px(12.0)).child("Active"))
                .child(div().flex_1())
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(6.0))
                        .child(Button::new("filter").ghost().label("Filter").small())
                        .child(Button::new("sort").ghost().label("Sort").small())
                        .child(Button::new("new").primary().label("New issue").small()),
                ),
        )
        // Section title
        .child(
            div()
                .px(px(44.0))
                .py(px(24.0))
                .flex()
                .items_end()
                .child(
                    div()
                        .flex_1()
                        .child(
                            div()
                                .text_size(px(23.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .child("Active issues"),
                        )
                        .child(
                            div()
                                .mt(px(5.0))
                                .text_size(px(12.0))
                                .text_color(rgb(0x777b83))
                                .child("Issues currently being worked on"),
                        ),
                )
                .child(Button::new("more").ghost().label("•••").xsmall()),
        )
        .child(issue_list())
}

fn issue_list() -> impl IntoElement {
    div()
        .flex_1()
        .overflow_hidden()
        .flex()
        .flex_col()
        // Tabs bar
        .child(
            div()
                .h(px(36.0))
                .flex()
                .items_center()
                .gap(px(18.0))
                .px(px(44.0))
                .border_b_1()
                .border_color(rgb(0x272a2f))
                .child(tab("All", true))
                .child(tab("Assigned to me", false))
                .child(tab("Created by me", false)),
        )
        // Issue groups
        .child(
            div()
                .flex_1()
                .px(px(44.0))
                .py(px(20.0))
                .flex()
                .flex_col()
                .gap(px(1.0))
                .child(group_header("ENGINEERING", 3))
                .child(issue(
                    "ENG-124",
                    "Fix authentication redirect",
                    "Backend",
                    "John",
                    "12m",
                    "HIGH",
                    0xd95c5c,
                ))
                .child(issue(
                    "ENG-123",
                    "Improve settings page",
                    "Frontend",
                    "Sarah",
                    "1h",
                    "MEDIUM",
                    0xd9a441,
                ))
                .child(issue(
                    "ENG-122",
                    "Update API documentation",
                    "Documentation",
                    "Alex",
                    "3h",
                    "LOW",
                    0x5b8fd8,
                ))
                .child(
                    div()
                        .mt(px(12.0))
                        .flex()
                        .flex_col()
                        .gap(px(1.0))
                        .child(group_header("PRODUCT", 2))
                        .child(issue(
                            "PRO-087",
                            "Redesign onboarding flow",
                            "Product",
                            "Maya",
                            "5h",
                            "MEDIUM",
                            0xd9a441,
                        ))
                        .child(issue(
                            "PRO-086",
                            "Add keyboard shortcuts",
                            "Desktop",
                            "David",
                            "1d",
                            "LOW",
                            0x5b8fd8,
                        )),
                )
                .child(
                    div()
                        .pt(px(12.0))
                        .text_size(px(11.0))
                        .text_color(rgb(0x555960))
                        .child("Showing 5 of 24 active issues"),
                ),
        )
}

fn tab(label: &'static str, active: bool) -> impl IntoElement {
    div()
        .h_full()
        .flex()
        .items_center()
        .text_size(px(12.0))
        .text_color(if active { rgb(0xe2e3e6) } else { rgb(0x696d75) })
        .when(active, |this| this.border_b_2().border_color(rgb(0x7c6ff2)))
        .child(label)
}

fn group_header(name: &'static str, count: u32) -> impl IntoElement {
    div()
        .px(px(8.0))
        .py(px(8.0))
        .flex()
        .items_center()
        .gap(px(7.0))
        .child(
            div()
                .text_size(px(10.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(0x777b83))
                .child(name),
        )
        .child(
            div()
                .text_size(px(10.0))
                .text_color(rgb(0x50545b))
                .child(count.to_string()),
        )
}

fn issue(
    id: &'static str,
    title: &'static str,
    team: &'static str,
    assignee: &'static str,
    updated: &'static str,
    priority: &'static str,
    priority_color: u32,
) -> impl IntoElement {
    div()
        .h(px(60.0))
        .flex()
        .items_center()
        .px(px(12.0))
        .rounded(px(7.0))
        .border_1()
        .border_color(rgb(0x202227))
        .bg(rgb(0x141619))
        .hover(|this| this.bg(rgb(0x1a1c20)).border_color(rgb(0x303339)))
        .child(
            div()
                .size(px(16.0))
                .rounded_full()
                .border_2()
                .border_color(rgb(priority_color))
                .mr(px(11.0)),
        )
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .gap(px(3.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::MEDIUM)
                                .child(title),
                        )
                        .child(
                            div()
                                .text_size(px(10.0))
                                .text_color(rgb(0x5f636b))
                                .child(id),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .text_size(px(10.0))
                        .text_color(rgb(0x656970))
                        .child(team)
                        .child("·")
                        .child(format!("Updated {updated} ago")),
                ),
        )
        .child(
            div()
                .px(px(7.0))
                .py(px(3.0))
                .rounded(px(4.0))
                .bg(rgb(priority_color))
                .text_color(rgb(0x101113))
                .text_size(px(9.0))
                .font_weight(FontWeight::SEMIBOLD)
                .child(priority),
        )
        .child(
            div()
                .ml(px(16.0))
                .size(px(26.0))
                .rounded_full()
                .bg(rgb(0x30343b))
                .flex()
                .items_center()
                .justify_center()
                .text_size(px(10.0))
                .text_color(rgb(0xc4c6cb))
                .child(assignee.chars().next().unwrap_or('?').to_string()),
        )
}

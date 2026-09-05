use std::borrow::Cow;

use anyhow::Result;
use rust_embed::RustEmbed;

use gpui_kit::*;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::component::{
    Root,
    Icon,
    IconName,
    Sizable,
    button::Button,
    button::ButtonVariants,
};

/// Application-specific assets embedded at compile time.
/// Falls back to gpui-kit's bundled icons for anything not found here.
#[derive(RustEmbed)]
#[folder = "assets"]
struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }
        // Try custom assets first.
        if let Some(f) = AppAssets::get(path) {
            return Ok(Some(f.data));
        }
        // Fall back to gpui-kit's bundled icons.
        gpui_kit::assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut files: Vec<SharedString> = AppAssets::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect();
        files.extend(gpui_kit::assets::Assets.list(path)?);
        Ok(files)
    }
}


struct App;

impl Render for App {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x0f1012))
            .text_color(rgb(0xe8e9ea))
            .child(
                div()
                    .size_full()
                    .flex()
                    .flex_row()
                    .child(sidebar())
                    .child(main_content()),
            )
    }
}

fn main() {
    let app = gpui_kit::application().with_assets(AppAssets);

    app.run(|cx| {
        gpui_kit::init(cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: point(px(0.0), px(0.0)),
                    size: size(px(1200.0), px(760.0)),
                })),
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|_| App);
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
            .expect("failed to open window");
    });
}

fn sidebar() -> impl IntoElement {
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


fn main_content() -> impl IntoElement {
    div()
        .flex_1()
        .h_full()
        .flex()
        .flex_col()
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

fn issue_list() -> impl IntoElement {
    div()
        .flex_1()
        .overflow_hidden()
        .flex()
        .flex_col()
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


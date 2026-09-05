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
}


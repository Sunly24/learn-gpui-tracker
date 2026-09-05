mod assets;
mod ui;

use assets::AppAssets;
use ui::{sidebar, main_content};

use gpui_kit::*;
use gpui_kit::component::Root;

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

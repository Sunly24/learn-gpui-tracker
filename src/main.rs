use gpui_kit::component::Root;
use gpui_kit::*;

struct App;

impl Render for App {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Hello, GPUI!")
    }
}

fn main() {
    let app = gpui_kit::application();

    app.run(|cx| {
        gpui_kit::init(cx);

        cx.open_window(
            WindowOptions::default(),
            |window, cx| {
                let view = cx.new(|_| App);
                cx.new(|cx| Root::new(view, window, cx))
            },
        )
            .expect("failed to open window");
    });
}

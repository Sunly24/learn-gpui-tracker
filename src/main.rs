use gpui::*;
use gpui_component::Root;

struct App;

impl Render for App {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Hello, GPUI!")
    }
}

fn main() {
    let app = gpui_platform::application();

    app.run(|cx| {
        gpui_component::init(cx);

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

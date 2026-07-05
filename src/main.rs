mod app;
mod filesystem;
mod desktop;
mod panes;

use app::App;

fn main() -> iced::Result {
    app::init_colors();
    iced::application("File Manager", App::update, App::view)
        .window(iced::window::Settings {
            size: iced::Size::new(1200.0, 800.0),
            decorations: true,
            position: iced::window::Position::Centered,
            ..Default::default()
        })
        .subscription(App::subscription)
        .run_with(App::new)
}

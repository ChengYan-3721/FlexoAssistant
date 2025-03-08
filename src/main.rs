use floem::kurbo::Size;
use floem::window::WindowConfig;
use floem::Application;

mod ui;
mod flexo;

fn main() {
    let size = Size {
        width: 420f64,
        height: 690f64,
    };
    let config = WindowConfig::default().size(size);

    Application::new()
        .window(move |_| ui::counter_view(), Option::from(config))
        .run()
}

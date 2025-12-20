mod app;
mod maps;

use std::io;
use app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    // create and run app
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

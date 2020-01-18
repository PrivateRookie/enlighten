mod api;
mod ui;

fn main() {
    log4rs::init_file("log.yml", Default::default()).unwrap();
    ui::render_ui();
}

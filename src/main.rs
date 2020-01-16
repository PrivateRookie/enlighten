use cursive::Cursive;

mod api;
mod ui;

fn main() {
    log4rs::init_file("log.yml", Default::default()).unwrap();
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    siv.add_layer(ui::render());
    siv.run();
}

use wgpu_noboiler::App;

fn main() {
    let application = App::new();
    application.input(|state, window| {
        true
    });
    application.run();
}
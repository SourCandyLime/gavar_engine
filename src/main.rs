mod app;

fn main() -> Result<(), app::AppError> {
    app::App::new().run()
}
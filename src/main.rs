#[macroquad::main("uju")]
async fn main() {
    let mut app = uju::app::App::new().await;
    app.run().await;
}

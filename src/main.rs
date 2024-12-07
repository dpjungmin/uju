#[macroquad::main("uju")]
async fn main() -> anyhow::Result<()> {
    let mut app = uju::app::App::new().await;
    app.run().await;
    Ok(())
}

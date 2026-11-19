//! `cargo run -p rporea --example hello [ポート]` → http://127.0.0.1:8768/
use rporea::reaxt::{app::App, react::VNode, RenderMode};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::args().nth(1).and_then(|p| p.parse().ok()).unwrap_or(8768);
    let app = Arc::new(
        App::new("RPoRea hello")
            .page("index.rs", RenderMode::Ssr, |_| VNode::element("h1").child(VNode::text("Hello from RPoRea")).build())
            .finish(),
    );
    println!("http://127.0.0.1:{port}/");
    rporea::serve(app, ([127, 0, 0, 1], port)).await.unwrap();
}

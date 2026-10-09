pub mod app;
pub mod components;
pub mod data;
pub mod engine;
pub mod platform;
pub mod screens;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    platform::run(App::default())
}

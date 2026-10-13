pub mod app;
pub mod audio;
pub mod data;
pub mod engine;
pub mod platform;
pub mod ratcn;
pub mod screens;
pub mod utils;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    platform::run(App::default())
}

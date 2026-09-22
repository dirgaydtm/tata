pub mod app;
pub mod components;
pub mod platform;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    platform::run(App::default())
}

pub mod genetic;
pub mod plot;
pub mod boot;
use boot::Boot;

fn main() {
    let boot = Boot::new();
    boot.boot();
}
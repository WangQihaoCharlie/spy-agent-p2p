mod core;
use core::ping;

fn main() {
    if let Err(e) = ping("127.0.0.1", 12345) {
        eprintln!("Error: {}", e);
    }
}

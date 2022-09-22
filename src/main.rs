fn main() {
    if let Err(e) = chat::get_args().and_then(chat::run) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

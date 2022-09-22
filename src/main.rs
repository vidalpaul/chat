fn main() {
    if let Err(e) = chat::run() {
    eprintln!("{}", e);
    std::process::exit(1);
    }
   }
use rs_a::startup::run;
fn main() {
    if let Err(e) = run() {
        eprint!("{}", e);
        std::process::exit(1);
    }
}

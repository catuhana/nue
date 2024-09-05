fn main() {
    if cfg!(windows) {
        panic!("Nue is not supported on Windows.");
    }
}

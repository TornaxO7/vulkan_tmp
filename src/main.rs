fn main() {
    if let Err(err) = vulkan::run() {
        panic!("{}", err);
    }
}

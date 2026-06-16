pub fn open_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(path)
}
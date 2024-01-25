pub fn clear_cons() -> () {
    print!("\x1B[2J\x1B[1;1H");
}
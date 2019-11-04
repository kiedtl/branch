pub fn info(stuff: String) {
    println!("=> INFO: {}", stuff)
}

pub fn error(stuff: String) {
    println!("!> ERR!: {}", stuff);
}

pub fn die(stuff: String) {
    error(stuff);
    error("previous error not recoverable; aborting.");
    std::process::exit(1);
}

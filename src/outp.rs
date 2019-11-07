pub fn debug(stuff: String) {
    log::debug!("=> DBG: {}", stuff);
}

pub fn info(stuff: String) {
    println!("=> INFO: {}", stuff)
}

pub fn error(stuff: String) {
    println!("!> ERR!: {}", stuff);
}

pub fn die(stuff: String) {
    error(stuff);
    error("previous error not recoverable; aborting.".to_string());
    std::process::exit(1);
}

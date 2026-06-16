#[cfg(windows)]
fn main() {
    simthing_mapeditor::run();
}

#[cfg(not(windows))]
fn main() {
    eprintln!("SimThing Studio requires Windows.");
    std::process::exit(1);
}

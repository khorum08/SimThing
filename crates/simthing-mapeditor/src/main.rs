#![cfg(windows)]

fn main() {
    simthing_mapeditor::run();
}

#[cfg(not(windows))]
fn main() {
    eprintln!("SimThing Studio PR1 requires Windows.");
    std::process::exit(1);
}

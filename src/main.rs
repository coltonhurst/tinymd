fn get_version() -> u16 {
    return 1000;
}

fn usage() {
    let the_version = get_version();
    println!("tinymd, a markdown compiler written by Colton Hurst");
    println!("Version {}", the_version);
}

fn main() {
    usage();
}

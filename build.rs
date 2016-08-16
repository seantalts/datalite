use std::process::Command;
use std::env::current_dir;


fn main() {
    let serd: String = format!("{}/vendor/serd-0.22.0",
                               current_dir().unwrap().display());

    Command::new("./waf")
        .arg("configure")
        .arg("--static")
        .current_dir(&serd)
        .status().unwrap();

    Command::new("./waf")
        .current_dir(&serd)
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}/build", serd);
    println!("cargo:rustc-link-lib=static=serd-0");
}

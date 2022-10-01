use std::process::Command;
use webbrowser::open;

fn main() {

    open("http://github.com/KLYN74R");

    //Let's also check docker version on machine
    let docker_version = String::from_utf8(Command::new("docker").arg("--version").output().unwrap().stdout).unwrap();

    println!("Docker version: {}",docker_version);

}
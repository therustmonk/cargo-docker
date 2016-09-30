//! A cargo subcommand for building project inside docker and get outputs back.

#[macro_use]
extern crate clap;

use std::env;
use std::process::Command;
use clap::{App, AppSettings, SubCommand, Arg};

fn main() {
    let app = App::new("cargo-count")
        .bin_name("cargo")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("docker")
            .version(concat!("v", crate_version!()))
            .author("Denis Kolodin <DenisKolodin@gmail.com>")
            .about("Build Rust code with Docker")
            .arg(Arg::with_name("image")
                .short("i")
                .long("image")
                .value_name("image")
                .help("Image to use for building")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("output")
                .help("Output directory")
                .takes_value(true)));

    let m = app.get_matches();

    if let Some(matches) = m.subcommand_matches("docker") {

        let p = env::current_dir().unwrap();
        let source_folder = format!("{}:/source", p.display());

        let output_folder = matches.value_of("output").unwrap_or("output");
        let target_folder = format!("{}/{}:/source/target", p.display(), output_folder);
        let cargo_folder = format!("{}/{}/.cargo:/root/.cargo", p.display(), output_folder);

        let image = matches.value_of("image").unwrap();

        let mut command = Command::new("docker")
            // Run new container
            .arg("run")
            // Allocate pseudo-tty
            .arg("-t")
            // Remove container after using
            .arg("--rm")
            // Attach virtual volume with sources
            .args(&["-v", &source_folder])
            .args(&["-v", &target_folder])
            .args(&["-v", &cargo_folder])
            .arg(image)
            .args(&["build", "--release"])
            .spawn()
            .expect("failed to execute docker");

        command.wait().expect("docker failed");
    }
}

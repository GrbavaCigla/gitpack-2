use std::env::set_current_dir;
use std::error::Error;
use std::path::Path;
use std::process::{Command, Output};

#[derive(Debug)]
pub enum BuildSystem {
    CMake,
    Make,
    Meson,
    Setup,
    Pipfile,
    Cargo,
}

pub fn check_build_system<P: AsRef<Path>>(path: &P) -> Option<BuildSystem> {
    let path = path.as_ref();

    let cmake_file = path.join("CMakeLists.txt").exists();
    let make_file = path.join("Makefile").exists();
    let meson_file = path.join("meson_options.txt").exists();
    let pip_file = path.join("Pipfile").exists();
    let setup_file = path.join("setup.py").exists();
    let cargo_file = path.join("Cargo.toml").exists();

    if make_file {
        return Some(BuildSystem::Make);
    } else if cmake_file {
        return Some(BuildSystem::CMake);
    } else if meson_file {
        return Some(BuildSystem::Meson);
    } else if pip_file {
        return Some(BuildSystem::Pipfile);
    } else if setup_file {
        return Some(BuildSystem::Setup);
    } else if cargo_file {
        return Some(BuildSystem::Cargo);
    }

    None
}

fn run_command(cmd: &str) -> Result<Output, std::io::Error> {
    let cmds_args: Vec<&str> = cmd.split(" ").collect();

    let command = cmds_args[0];
    let args = &cmds_args[1..];

    let output = Command::new(command).args(args).output();

    output
}

pub fn run_build_cmd<P: AsRef<Path>>(
    path: &P,
    bs: &BuildSystem,
) -> Result<Option<Output>, Box<dyn Error>> {

    let commands = match bs {
        BuildSystem::CMake => vec!["cmake .", "make"],
        BuildSystem::Make => vec!["make"],
        BuildSystem::Meson => vec!["meson .", "ninja"],
        BuildSystem::Cargo => vec!["cargo build"],
        BuildSystem::Pipfile => vec![""], // TODO: This command
        BuildSystem::Setup => vec!["python setup.py sdist bdist_wheel"],
    };

    set_current_dir(&path)?;

    let mut output = None;
    for command in commands.iter() {
        output = Some(run_command(command)?);
    }

    Ok(output)
}

pub fn run_install_cmd<P: AsRef<Path>>(
    path: &P,
    bs: &BuildSystem,
) -> Result<Option<Output>, Box<dyn Error>> {
    
    let commands = match bs {
        BuildSystem::CMake => vec!["make install"],
        BuildSystem::Make => vec!["make install"],
        BuildSystem::Meson => vec!["ninja install"],
        BuildSystem::Cargo => vec!["cargo install --path ."],
        BuildSystem::Pipfile => vec![""],  // TODO: Have to make sure command is right
        BuildSystem::Setup => vec![""],    // TODO: I have to check exact command
    };

    set_current_dir(&path)?;

    let mut output = None;
    for command in commands.iter() {
        output = Some(run_command(command)?);
    }

    Ok(output)
}

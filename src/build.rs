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

pub fn run_build_cmd<P: AsRef<Path>>(path: &P, bs: BuildSystem) -> Result<Output, Box<dyn Error>> {
    let command = match bs {
        BuildSystem::CMake => "cmake . && make",
        BuildSystem::Make => "make",
        BuildSystem::Meson => "meson . && ninja",
        BuildSystem::Cargo => "cargo build",
        BuildSystem::Pipfile => "",
        BuildSystem::Setup => "pip install .",
    };

    set_current_dir(&path)?;
    let output = Command::new(command).output()?;

    Ok(output)
}

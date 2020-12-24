extern crate config;
use colored::Colorize;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, RemoteCallbacks, Repository};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use std::process::exit;
use crate::error::GPError;

mod db;
mod error;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "gitpack",
    about = "Gitpack v2, written in rust instead of C, package manager."
)]
enum Gitpack {
    #[structopt(name = "install", about = "Install git repository")]
    Install {
        #[structopt(help = "Package to install")]
        package: String,
        #[structopt(long, short, help = "Use master branch")]
        master: bool,
    },
    #[structopt(name = "update", about = "Update all packages", )]
    Update {},
    #[structopt(name = "list", about = "List all packages")]
    List {},
}

fn checkout_latest(repo: &Repository) -> Option<String> {
    let tags = match repo.tag_names(None) {
        Ok(tags) => tags,
        Err(_) => return None,
    };

    if tags.len() < 1 {
        return None;
    }

    let latest = match tags.get(tags.len() - 1) {
        Some(latest) => latest,
        None => return None,
    };

    let spec = format!("refs/tags/{}", latest);

    let spec = match repo.revparse_single(&spec) {
        Ok(spec) => spec,
        Err(_) => return None,
    };

    let mut checkout_opts = git2::build::CheckoutBuilder::new();
    checkout_opts.force().use_theirs(true);

    match repo.checkout_tree(&spec, Some(&mut checkout_opts)) {
        Ok(a) => a,
        Err(e) => {
            error!("{}", e);
            return None;
        }
    };

    return Some(String::from(latest));
}

fn clone(url:&str, path: &Path, master: bool) -> (Repository, String) {
    let mut cb = RemoteCallbacks::new();
    cb.transfer_progress(|stats| {
        println!("{}/{}", stats.received_objects(), stats.total_objects());
        true
    });

    let mut co = CheckoutBuilder::new();
    co.force().use_theirs(true).progress(|path, cur, total| {
        println!("{:?} {} {}", path, cur, total)
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    let rb = RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(url, path);


    let rb = match rb {
        Ok(repo) => repo,
        Err(e) => match e.code() {
            git2::ErrorCode::Exists => Repository::open(path).unwrap(),
            _ => custompanic!("Failed to clone the repository")
        },
    };


    let mut latest: Option<String> = None;
    if !master {
        latest = checkout_latest(&rb);
    }

    let version = match latest {
        Some(ver) => ver,
        None => String::from("master"),
    };

    (rb, version)
}

fn update(cache_dir: &str, database: &db::PackageDB) {
    for pkg in database.list().iter() {
        let mut path = PathBuf::from(cache_dir);
        path.push(&pkg.name);

        let mut master = false;
        if pkg.version == "master" {
            master = true;
        }

        info!("Updating package {}", pkg.name);
        clone(&pkg.url, &path, master);
        info!("Finished updating package {}", pkg.name);
    }
}

fn list(database: &db::PackageDB) {
    info!("Fetching list of packages");
    for pkg in database.list().iter() {
        println!(" {}-{}", pkg.name.bold(), pkg.version.bold().green());
    }
}

fn install(
    package_name: &str,
    sources: &Vec<config::Value>,
    cache_dir: &str,
    database: &db::PackageDB,
    master: bool,
) {
    let mut found_repo = false;

    for source in sources {
        let temp_url = format!("{}{}", source, package_name);
        let res = reqwest::blocking::get(&temp_url).unwrap();

        if res.status() == 200 {
            found_repo = true;
            info!("Found the repository at {}", temp_url);

            let mut path = PathBuf::from(cache_dir);
            path.push(package_name);

            let (_repo, version) = clone(&temp_url, &path, master);

            info!("Installing version {}", version);

            let _db_package = match database.get(package_name) {
                Some(package) => package,
                None => {
                    let pkg = db::Package {
                        name: String::from(package_name),
                        url: temp_url,
                        version: version,
                    };
                    database.add(&pkg);
                    pkg
                }
            };

            break;
        }
    }
    if !found_repo {
        error!("Failed to fetch the repository :(");
    }
}

fn main() {
    let opt = Gitpack::from_args();

    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("/etc/gitpack.toml"))
        .escape("There is no config file at /etc/gitpack.toml.");

    let sources = settings
        .get_array("sources")
        .escape("There is no sources in config or sources is not an array.");

    let db_path = settings
        .get_str("db_path")
        .escape("There is no db_path in config or db_path is not a string.");

    let cache_dir = settings
        .get_str("cache_dir")
        .escape("There is no cache_dir in config or cache_dir is not a string");

    let package_db = db::PackageDB::new(PathBuf::from(db_path));
    package_db.create_db();

    match opt {
        Gitpack::Install { package, master } => {
            install(&package, &sources, &cache_dir, &package_db, master)
        }
        Gitpack::Update {} => update(&cache_dir, &package_db),
        Gitpack::List {} => list(&package_db),
    }
}

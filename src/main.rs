extern crate config;
use git2::Repository;
use std::path::PathBuf;
use structopt::StructOpt;

mod db;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "gitpack",
    about = "Gitpack v2, written in rust instead of C, package manager."
)]
enum Gitpack {
    #[structopt(name = "install")]
    Install {
        #[structopt(help = "Package to install")]
        package: String,

        #[structopt(long, short, help = "Use master branch")]
        master: bool,
    },
    #[structopt(name = "update")]
    Update {},
}

fn checkout_latest(repo: Repository) -> Option<String> {
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
            println!("{}", e);
            return None;
        }
    };

    return Some(String::from(latest));
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
            println!("[:] Found the repository at {}", temp_url);

            let mut path = PathBuf::from(cache_dir);
            path.push(package_name);

            let repo = match Repository::clone_recurse(&temp_url, &path) {
                Ok(repo) => repo,
                Err(e) => match e.code() {
                    git2::ErrorCode::Exists => Repository::open(&path).unwrap(),
                    _ => {
                        eprintln!("[!] Failed to clone the repository");
                        std::process::exit(1);
                    }
                },
            };

            let mut latest: Option<String> = None;
            if !master {
                latest = checkout_latest(repo);
            }

            let version = match latest {
                Some(ver) => ver,
                None => String::from("master"),
            };

            println!("[:] Installing version {}", version);

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
        eprintln!("[!] Failed to fetch the repository :(");
    }
}

fn main() {
    let opt = Gitpack::from_args();

    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name("/etc/gitpack.toml"))
        .expect("There is no config file at /etc/gitpack.toml.");

    let sources = settings
        .get_array("sources")
        .expect("There is no sources in config or sources is not an array.");

    let db_path = settings
        .get_str("db_path")
        .expect("There is no db_path in config or db_path is not a string.");

    let cache_dir = settings
        .get_str("cache_dir")
        .expect("There is no cache_dir in config or cache_dir is not a string");

    let package_db = db::PackageDB::new(PathBuf::from(db_path));
    package_db.create_db();

    match opt {
        Gitpack::Install { package, master } => {
            install(&package, &sources, &cache_dir, &package_db, master)
        }
        Gitpack::Update {} => (),
    }
}

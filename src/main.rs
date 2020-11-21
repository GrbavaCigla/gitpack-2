extern crate config;
use git2::Repository;
use structopt::StructOpt;
use std::path::PathBuf;

mod db;

#[derive(StructOpt, Debug)]
#[structopt(name = "gitpack", about = "Gitpack v2, written in rust instead of C, package manager.")]
enum Gitpack {
    #[structopt(name = "install")]
    Install {
        #[structopt(help = "Package to install")]
        package: String
    },
    #[structopt(name = "update")]
    Update {},
}

fn checkout_latest(repo: Repository) {
    let tags = match repo.tag_names(None) {
        Ok(tags ) => tags,
        Err(_) => return
    };

    let latest = match tags.get(tags.len()-1) {
        Some(latest) => latest,
        None => return
    };

    let spec = format!("refs/tags/{}", latest);

    let spec = match repo.revparse_single(&spec) {
        Ok(spec) => spec,
        Err(_) => return
    };

    match repo.checkout_tree(&spec, None) {
        Ok(a) => a,
        Err(_) => return
    };
}

fn install(package: &str, sources: &Vec<config::Value>, cache_dir: &str, master: bool) {

    for source in sources{
        let temp_url = format!("{}{}", source, package);
        let res = reqwest::blocking::get(&temp_url).unwrap();

        if res.status() == 200 {
            println!("{}", temp_url);

            let mut path = PathBuf::from(cache_dir);
            path.push(package);

            let repo = match Repository::clone(&temp_url, &path) {
                Ok(repo) => repo,
                Err(e) => match e.code() {
                    git2::ErrorCode::Exists => Repository::open(&path).unwrap(),
                    _ => panic!("failed to clone: {}", e)
                },
            };

            if !master {
                checkout_latest(repo);
            }


            break;
        }
    }
}

fn main(){
    let opt = Gitpack::from_args();

    let mut settings = config::Config::default();


    settings.merge(config::File::with_name("/etc/gitpack.toml"))
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

    match opt {
        Gitpack::Install { package } => install(&package, &sources, &cache_dir, true),
        Gitpack::Update { } => (),
    }
}

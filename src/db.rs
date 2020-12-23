extern crate rusqlite;

use crate::error::GPError;

use rusqlite::{params, Connection, NO_PARAMS};
use std::option::Option;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PackageDB {
    path: PathBuf,
    conn: Connection,
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub url: String,
    pub version: String,
}

impl PackageDB {
    pub fn new(path: PathBuf) -> PackageDB {
        let conn =
            Connection::open(&path).escape("Couldn't connect to database. Check the permissions.");

        PackageDB {
            path: path,
            conn: conn,
        }
    }

    pub fn create_db(&self) {
        let _ = self
            .conn
            .execute(
                "CREATE TABLE IF NOT EXISTS Packages (\
            id INTEGER PRIMARY KEY AUTOINCREMENT, \
            name TEXT, \
            url TEXT UNIQUE, \
            version TEXT \
            );",
                NO_PARAMS,
            )
            .unwrap();
    }

    pub fn add(&self, pkg: &Package) {
        let _ = self
            .conn
            .execute(
                "INSERT INTO Packages \
            (name, url, version) VALUES (?,?,?)",
                params![pkg.name, pkg.url, pkg.version],
            )
            .escape("Failed to add package into the database");
    }

    pub fn get(&self, name: &str) -> Option<Package> {
        let mut pkg_prep = self
            .conn
            .prepare("SELECT name, url, version FROM Packages WHERE name=?;")
            .escape("Failed to fetch data from database");

        let pkg_iter = pkg_prep
            .query_map(params![name], |row| {
                Ok(Package {
                    name: row.get(0)?,
                    url: row.get(1)?,
                    version: row.get(2)?,
                })
            })
            .unwrap();
        let mut pkg: Option<Package> = None;
        for package in pkg_iter {
            pkg = Some(package.escape("Failed to fetch data from database"));
            break;
        }
        return pkg;
    }

    pub fn list(&self) -> Vec<Package> {
        let mut pkg_prep = self
            .conn
            .prepare("SELECT name, url, version FROM Packages")
            .escape("Failed to fetch data from database");

        let pkg_iter = pkg_prep
            .query_map(NO_PARAMS, |row| {
                Ok(Package {
                    name: row.get(0)?,
                    url: row.get(1)?,
                    version: row.get(2)?,
                })
            })
            .unwrap();
        

        let mut pkgs: Vec<Package> = vec![];

        for package in pkg_iter {
            let pkg = package.escape("Failed to fetch data from database");
            pkgs.push(pkg);
        }
        return pkgs;
    }
}

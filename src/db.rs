extern crate rusqlite;

use rusqlite::{Connection, params, NO_PARAMS, Row};
use std::path::PathBuf;
use std::option::Option;

#[derive(Debug)]
pub struct PackageDB {
    path: PathBuf,
    conn: Connection
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub url: String,
    pub version: String,
}

impl PackageDB {
    pub fn new(path: PathBuf) -> PackageDB {
        let conn = Connection::open(&path)
            .expect("Couldn't connect to database. Check the permissions.");

        PackageDB {
            path: path, 
            conn: conn
        }
    }

    pub fn create_db(&self) {
        let _ = self.conn.execute("CREATE TABLE IF NOT EXISTS Packages (\
            id INTEGER PRIMARY KEY AUTOINCREMENT, \
            name TEXT, \
            url TEXT UNIQUE, \
            version TEXT \
            );", params![]).unwrap();
    }

    pub fn add(&self, pkg: &Package) {
        let _ = self.conn.execute("INSERT INTO Packages \
            (name, url, version) VALUES (?,?,?)", params![pkg.name, pkg.url, pkg.version])
            .expect("Failed to add package into the database");
    }

    pub fn get(&self, _name: &str) -> Option<Package> {
        let mut pkg_prep = self.conn
            .prepare("SELECT name, url, version FROM Packages WHERE name=?;")
            .expect("Failed to fetch data from database");
        
        let pkg_iter = pkg_prep.query_map(params![_name],
            |row| {
                Ok(
                    Package{
                        name: row.get(0)?,
                        url: row.get(1)?,
                        version: row.get(2)?
                    }
                )
            }
        ).unwrap();

        let mut b: Option<Package> = None;
        for package in pkg_iter {
            b = Some(package.expect("Failed to fetch data from database"));
            break;
        }
        
        return b;
    }
}

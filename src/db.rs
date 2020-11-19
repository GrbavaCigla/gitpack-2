extern crate rusqlite;

use rusqlite::{Connection, params};
use std::path::PathBuf;
use std::result::Result;

#[derive(Debug)]
struct PackageDB {
    path: PathBuf,
    conn: Connection
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
            name TEXT,\
            version TEXT,\
            );", params![]);
    }

    pub fn add(&self, name: &String, version: &String) {
        let _ = self.conn.execute("INSERT INTO Packages \
            (name, version) VALUES (?,?)", params![name, version]);
    }

    pub fn get(&self, name: &String) -> String {
        let versions_prep = self.conn
            .prepare("SELECT version FROM Packages")
            .expect("Failed to fetch data from database");
        
        let version_iter = versions_prep
            .query_map(params![], |row| {
                Ok(row.get_unwrap(0))
            })
            .expect("Failed to fetch data from database");
        
        for version in version_iter {
            let _ = version.unwrap();
        };

        String::new()
    }
}
use std::path::PathBuf;

use redb::Database;

pub fn init_db(p: &mut PathBuf) -> Result<Database, std::io::Error> {
    p.push("mail.db");
    match Database::create(p) {
        Ok(db) => {
            Ok(db)
        }
        Err(e) => {
            println!("{:?}", e);
            Err(std::io::Error::other(e))
        }
    }
}

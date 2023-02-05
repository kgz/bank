/**
 * @description Add default superuser.
 * @author Mat Frayne
 * @created 2023-01-08
 */

use crate::database::database;
use crate::database::database::Database;

pub fn run () {
    let db = database::new().unwrap();

    // alter users  table add column level being int (100) not null default 0

    
    let q:&str = "INSERT INTO `users` (`username`, `email`, `password`) VALUES ('?', '?', '?');";
    let args: Vec<&str> = vec!["SA", "sa@localhost", "password"];
    let q:String = db.prepare(q, &args);
    let r = db.query(&q);
    println!("result: {:?}", r);
}
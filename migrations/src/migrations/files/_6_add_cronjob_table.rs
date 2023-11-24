/**
 * @description Add default superuser.
 * @author Mat Frayne
 * @created 2023-02-12
 */

use crate::database::database;
use crate::database::database::Database;

pub fn run () {
    let db = database::new().unwrap();

    // alter users  table add column level being int (100) not null default 0
// create table cronjobs id: autoincrement, name: string, command: string, schedule: string, active: bool, created_at: datetime, updated_at: datetime
    
    let q:&str = "CREATE table cronjobs (
        id int NOT NULL AUTO_INCREMENT PRIMARY KEY,
        name varchar(255) NOT NULL UNIQUE, 
        schedule varchar(255) NOT NULL, 
        active bool, 
        created_at datetime, 
        updated_at datetime
    )";
    let r = db.query(&q, None);
    println!("result: {:?}", r);
}
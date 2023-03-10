/**
 * @description This file contains the migration for creating the user table.
 * @author Mat Frayne
 * @created 2023-01-07
 */

use crate::database::database;
use crate::database::database::Database;

pub fn run () {
    let db = database::new().unwrap();

    // create a users  table with username, email, password, created_at, updated_at, locked_until, failed_login_attempts, and last_login_attempt, and last_login_attempt_ip, deleted_at

    let q:&str = "CREATE TABLE `users` (
        `id` int(11) NOT NULL AUTO_INCREMENT,
        `username` varchar(255) NOT NULL,
        `email` varchar(255) NOT NULL,
        `password` varchar(255) NOT NULL,
        `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
        `updated_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
        `locked_until` datetime,
        `failed_login_attempts` int(11) NOT NULL DEFAULT 0,
        `last_login_attempt` datetime,
        `last_login_attempt_ip` varchar(255),
        `deleted_at` datetime,
        PRIMARY KEY (`id`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;";

        // let q:&str = "INSERT INTO `test` (`name`) VALUES ('?')";
    // let args: Vec<&str> = vec!["hello"];
    // let q:String = db.prepare(q, &args);
    let _r = db.query(&q, None);
    
}
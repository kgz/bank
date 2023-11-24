// use crate::files;

// use super::migrations::files;

// vec of files 

use crate::migrations;

pub static MIGRATIONS: &[(&str, fn())] = &[
    ("a" , migrations::files::_1_a::run),
    ("b" , migrations::files::_2_b::run),
    ("create_user_table", migrations::files::_3_create_user_table::run),
    ("add_default_admin_user", migrations::files::_5_add_default_admin_user::run),
    ("6_add_cronjob_table", migrations::files::_6_add_cronjob_table::run)
    // auto load all in migrations folder
    
];
// hash map of files

use crate::database::database;

pub fn run () {
    println!("Hello, world!");
    let db = database::new().unwrap();
        // let q:&str = "INSERT INTO `test` (`name`) VALUES ('?')";
    // let args: Vec<&str> = vec!["hello"];
    // let q:String = db.prepare(q, &args);
    // let r = db.query(&q);
    
}
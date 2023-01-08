use database::database::Database;

pub mod database {
    pub mod database;
}

pub mod migrations{
    pub mod __all__;

    pub mod files{
        automod::dir!(pub "src/migrations/files"); 

    }
}


fn has_been_run(name: String) -> bool {
    let db = database::database::new().unwrap();
    let q = "SELECT * FROM `migrations` WHERE `name` = '?'";
    let q = db.prepare(q, &[&name]);
    let res: database::database::Ret = db.query(&q);
    if res.result.len() > 0 {
        return true;
    }
    return false;
}

pub fn run_migrations() {
    
    // fn add_function(name: String, f: VoidFnPtr) {
    //     if !has_been_run(name) {
    //         // functions.insert(name, f);
    //     }
    // }

    // functions.insert("a".to_string(), Box::new(files::a::run));
    let functions = migrations::__all__::MIGRATIONS;

    // confirm there are no duplicate names or functions
    let mut names: Vec<&str> = Vec::new();
    let mut funcs: Vec<&fn()> = Vec::new();

    for (name, f) in functions {
        if names.contains(&name) {
            panic!("Duplicate migration name: {}", name);
        }
        names.push(name);

        if funcs.contains(&f) {
            panic!("Duplicate migration function: {}", name);
        }
        funcs.push(&f);
    }

    // print the type of `functions`
    for (n, f )in functions {
        // get function name
        if !has_been_run(n.to_string()) {
            println!("Running migration: {}", n);
            f();

            // add to migrations table
            let db = database::database::new().unwrap();
            let q = "INSERT INTO `migrations` (`name`) VALUES ('?')";
            let q = db.prepare(q, &[&n]);
            let res: database::database::Ret = db.query(&q);
            println!("res: {:?}", res);

        }

        // add to migrations table

    }
    //     // get function name
    //     let name = f.0;
    //     println!("name: {}", name);
    // }

    // let res = functions.get("a").unwrap();

    // // if res
    // if let Some(f) = functions.get("a") {
    //     f();
    // }
    // println!("{}", res);



}
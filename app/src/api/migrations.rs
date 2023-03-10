use migrations::database::{
    database::Database,
    database::new,
};



#[allow(dead_code)]
pub async fn get_migrations() -> Result<String, warp::Rejection> {
    let db = new().unwrap();
    let q:&str = "SELECT * FROM `migrations`";
    let r = db.query(q, None);

    // get headers from db
    // print r
    println!("r: {:?}", r);

    // warp::reply::with::header("Content-Type", "application/json")
    // 404/
    // return Err(warp::reject::not_found());

    // ret to jsonstring
    // set headers
    return Ok(db.jsonify(r));
}
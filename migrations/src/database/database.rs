use lazy_static::__Deref;
/**
 * Database module
 * @module database
 * @version 1.0.0
 * @description Database module
 * @author Mat Frayne
 */
use mysql::prelude::Queryable;
use regex::Regex;

pub trait Database {
    fn query(&self, query: &str, params:Option<&[&str]>) -> Ret;
    fn jsonify(&self, ret: Ret) -> String;
    fn sanitize(&self, query: &str) -> String {
        //escape query
        // as raw string
        query.to_string()
    }

    fn fetch(query: &str, args: Vec<&str>) -> Result<Ret, mysql::Error> {
        let db = self::new()?;
        let result = db.query(query, Some(&args));

        // let q: &str = query;
        // let q: String = db.prepare(q, &args);
        // println!("q: {}", q);
        // let result = db.query(&q, None);
        Ok(result)
    }

    /// Returns a prepared query string for mysql.
    /// Arguments replace ? in the query string in the order they are passed
    /// the ? character can be escaped with 2 backslashes
    ///
    /// ```
    ///     $db = new DB();
    ///     $query = "INSERT INTO `test` (`name`) VALUES ('?')";
    ///     $args = ["hello"];
    ///     $query = $db->prepare($query, $args);
    ///     assert_eq($query, "INSERT INTO `test` (`name`) VALUES ('hello')");
    /// ```
    fn prepare(&self, query: &str, params: &[&str]) -> String {
        let mut query = query.to_string();
        query = query.to_string();
        let search_regex_replacement = [
            // search string     search regex        sql replacement regex
            ["\u{0}",          "\\x00",            "\\\\0"],
            ["'",               "'",               "\\\\'"],
            ["\"",              "\"",              "\\\\\""],
            ["\x08",            "\\x08",            "\\\\b"],
            ["\n",              "\\n",              "\\\\n"],
            ["\r",              "\\r",              "\\\\r"],
            ["\t",              "\\t",              "\\\\t"],
            ["\x1A",            "\\x1A",            "\\\\Z"],
            ["\\",              "\\\\",             "\\\\\\\\"],
        ];
        for param in params {
            let mut repr: String = param.to_string(); 
           

            let mut i = 0;
            let mut escaped = false;
            for c in query.chars() {
                if c == '?' && !escaped {
                    query.replace_range(i..i + 1, &repr.as_str());
                    break;
                }
                if c == '\\' {
                    escaped = !escaped;
                } else {
                    escaped = false;
                }
                i += 1;
            }

            // replace \\? with ? in query
            query = query.replace("\\?", "?");
        }
        query.to_string()
    }
}

pub struct DB {
    pub conn: mysql::PooledConn,
}
#[derive(Debug)]
pub struct Ret {
    pub last: u64,
    pub affected: u64,
    pub results: Vec<mysql::Row>,
    pub headers: Vec<std::string::String>, // impliment to json string
}

impl Database for DB {
    fn query(&self, query: &str, params: Option<&[&str]>) -> Ret {
        println!("query: {}", query);
        let url = "mysql://root@localhost:3306/bank";
        let conn: Result<mysql::Pool, mysql::Error> = mysql::Pool::new(url);

        if let Err(_err) = conn {
            // print query
            println!("Query: {}", query);
            println!("PANIC Error: {}", _err);
            panic!("PANIC Error: {}", _err)
        }
        // assert type of conn
        let conn: mysql::Pool = conn.unwrap();

        let pool = conn.get_conn();
        if let Err(_err) = pool {
            // print query
            println!("Query: {}", query);
            println!("PANIC Error: {}", _err);
            panic!("PANIC Error: {}", _err)
        }
        // check if error
        let mut pool: mysql::PooledConn = pool.unwrap();
        // let result: Vec<mysql::Row> = pool.query(query).unwrap();
        
        let results: Result<Vec<mysql::Row>, mysql::Error>;
        if params.is_some(){
            println!("params: {:?}", params.unwrap().to_vec());
            results = pool.exec(query, params.unwrap().to_vec());
        }
        else {
            println!("params: None");
            results = pool.query(query);
        }
        // let q = pool.prep(query).unwrap();
        // println!("q: {:?}", q);
        // lastr run query
        if let Err(_err) = results {
            // print query
            println!("Query: {}", query);
            println!("PANIC Error: {}", _err);
            return Ret {
                last: 0,
                affected: 0,
                results: Vec::new(),
                headers: Vec::new(),
            };
        }

        let results: Vec<mysql::Row> = results.unwrap();
        let last = pool.last_insert_id();
        let affected = pool.affected_rows();
        let mut headers = Vec::new();

        if results.len() == 0 {
            let out = Ret {
                last,
                affected,
                results,
                headers,
            };
            return out;
        }

        let col_length = results[0].columns().len();
        for i in 0..col_length {
            let col = results[0].to_owned();
            let col = col.columns();
            let name = col[i].name_str();
            let name = format!("{}", name);
            headers.push(name);
        }

        let out = Ret {
            last: last,
            affected: affected,
            results,
            headers,
        };

        return out;
    }

    fn jsonify(&self, ret: Ret) -> String {
        let keys = ret.headers.clone();

        let r = ret.results;
        let mut out = String::from("[");
        for row in r {
            let keys = keys.clone();
            let mut row_out = String::from("{");
            let mut i = 0;
            for key in &keys {
                let val: Option<String> = row.get(i);
                let val = val.clone().unwrap();
                let val = val.replace("\"", "\\\"");
                let val = val.replace("\'", "\\\'");
                let val = val.replace("\n", "\\n");
                let val = val.replace("\r", "\\r");
                let val = val.replace("\t", "\\t");
                let val = val.replace("\x08", "\\b");
                let val = val.replace("\x0c", "\\f");
                let val = val.replace("\x00", "\\0");
                let val = val.replace("\x1a", "\\Z");
                row_out = row_out + &format!("\"{}\":\"{}\",", key, val);
                i += 1;
            }
            row_out.pop();
            row_out = row_out + "}";
            out = out + &row_out + ",";
        }

        out.pop();
        out = out + "]";

        return out;
    }
}

#[allow(dead_code)]
pub fn new() -> Result<DB, mysql::Error> {
    let url = "mysql://root@localhost:3306/bank";
    let conn: Result<mysql::Pool, mysql::Error> = mysql::Pool::new(url);

    if let Err(_err) = conn {
        println!("PANIC Error: {}", _err);
        panic!("PANIC Error: {}", _err)
    }
    // assert type of conn
    let conn: mysql::Pool = conn.unwrap();

    let pool = conn.get_conn()?;

    return Ok(DB { conn: pool });
}


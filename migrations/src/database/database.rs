use std::{collections::HashMap, borrow::Cow};

/**
 * Database module
 * @module database
 * @version 1.0.0
 * @description Database module
 * @author Mat Frayne
 */
use mysql::{prelude::Queryable, serde_json::{self, json}};
use serde::{Serialize, Deserialize};

pub trait Database {
    fn query(&self, query: &str) -> Ret;
    fn jsonify(&self, ret: Ret) -> String;
    fn sanitize(&self, query: &str) -> String {
        //escape query
        // as raw string
        query.to_string()
    }

    fn prepare(&self, query: &str, params: &[&str]) -> String {
        // format query
        let mut query = query.to_string();
        query = query.to_string();
        // format params
        for param in params {

            let param = param.to_string();
            let param = param.replace("'", "\\'");
            let param = param.replace("\"", "\\\"");
            let param = param.replace("%", "\\%");
            let param = param.replace("_", "\\_");
            let param = param.replace("\\", "\\\\");
            let param = param.replace("\0", "\\0");
            let param = param.replace("\n", "\\n");
            let param = param.replace("\r", "\\r");
            let param = param.replace("\x1a", "\\Z");
            let param = param.replace("\t", "\\t");
            let param = param.replace("\x08", "\\b");
            let param = param.replace("\x0c", "\\f");
            let param = param.replace("\x00", "\\0");
            let param = param.replace("\x1a", "\\Z");
            let param = param.replace("\t", "\\t");
            let param = param.replace("\x08", "\\b");
            let param = param.replace("\x0c", "\\f");
            let param = param.replace("\x00", "\\0");
            let param = param.replace("\x1a", "\\Z");
            let param = param.replace("\t", "\\t");
            let param = param.replace("\x08", "\\b");
            let param = param.replace("\x0c", "\\f");
            let param = param.replace("\x00", "\\0");
            let param = param.replace("\x1a", "\\Z");
            let param = param.replace("\t", "\\t");
            let param = param.replace("\x08", "\\b");
            let param = param.replace("\x0c", "\\f");
            let param = param.replace("\x00", "\\0");
            let param = param.replace("\x1a", "\\Z");
            let param = param.replace("\t", "\\t");
            let param = param.replace("\x08", "\\b");
            let param = param.replace("\x0c", "\\f");
            let param = param.replace("\x00", "\\0");
            let param = param.replace("\x1a", "\\Z");
            let param = param.replace("\t", "\\t");
            let param = param.replace("\x08", "\\b");
            let param = param.replace("\x0c", "\\f");
            let param = param.replace("\x00", "\\0");
            let param = param.replace("\x1a", "\\Z");
            let param = param.replace("\t", "\\t");
            let param = param.replace("\x08", "\\b");
            let param = param.replace("\x0c", "\\f");
            let param = param.replace("\x00", "\\0");
            let param = param.replace("\x1a", "\\Z");
            let param = param.replace("\t", "\\t");
            let param = param.replace("\x08", "\\b");
            let param = param.replace("\x0c", "\\f");
            query = query.replacen("?", &param, 1);

            // replace ? with index of i with param


        }
        query.to_string()
    }
}

pub struct DB {
    pub conn: mysql::PooledConn
}
#[derive(Debug)]
pub struct Ret {
    pub last: u64,
    pub affected: u64,
    pub result: Vec<mysql::Row>,
    pub headers: Vec<std::string::String>
    // impliment to json string
}

impl Database for DB {
    fn query(&self, query: &str) -> Ret {
        
        let url = "mysql://root@localhost:3306/bank";
        let conn:  Result<mysql::Pool, mysql::Error> = mysql::Pool::new(url);
    
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
        let result: Vec<mysql::Row> = pool.query(query).unwrap();
        let last = pool.last_insert_id();
        let affected = pool.affected_rows();
        let mut headers = Vec::new();

        if(result.len() == 0) {
            let out = Ret {
                last: last,
                affected: affected,
                result, 
                headers
            };
            return out;
        }

        let col_length = result[0].columns().len();
        for i in 0..col_length {
            
            let col = result[0].to_owned();
            let col = col.columns();
            let name = col[i].name_str();
            let name = format!("{}", name);
            headers.push(name);
        }

      


        println!("Last Insert ID: {}", last);
        println!("Affected Rows: {}", affected);
        
        let out = Ret {
            last: last,
            affected: affected,
            result, 
            headers

        };

        
        return out;
    }

    fn jsonify(&self, ret:Ret) -> String {

        let keys = ret.headers.clone();

        let r = ret.result;
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
                i +=1;
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
pub fn new() -> Result<DB, mysql::Error>{
    let url = "mysql://root@localhost:3306/bank";
    let conn:  Result<mysql::Pool, mysql::Error> = mysql::Pool::new(url);

    if let Err(_err) = conn {
        println!("PANIC Error: {}", _err);
        panic!("PANIC Error: {}", _err)
    }
    // assert type of conn
    let conn: mysql::Pool = conn.unwrap();

    let pool = conn.get_conn()?;

    return Ok(DB { conn: pool });
}
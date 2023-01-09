pub mod roles {

    fn get_roles() -> Vec<(&'static str, i32)> {
        // let roles = [
        //     ("a", 0), 
        //     ("b", 1),
        //     ("c", 1),
        //     ("d", 1),
        //     ("e", 0),
        //     ("f", 1),
        //     ("g", 0),
        //     ("h", 1),
        //     ("i", 1),
        //     ("j", 1),
        //     ("k", 0),
        // ];
        let roles = vec![
            ("a", 0), 
            ("b", 1),
            ("c", 1),
            ("d", 1),
            ("e", 0),
            ("f", 1),
            ("g", 0),
            ("h", 1),
            ("i", 1),
            ("j", 1),
            ("k", 0),
        ];
        roles
    }
    fn generate_code() -> String {
        let roles = get_roles();

        // let values: Vec<i32> = roles.iter().map(|(_, v)| v).collect();
        let mut values: Vec<i32> = Vec::new();
        for (_, v) in roles {
            values.push(v);
        }

        println!("{:?}", values);
        let mut code = "".to_string();
        let mut last = None;
        for x in values {
            if last.is_none() {
                last = Some(x);
                code.push_str(&x.to_string());
                continue;
            }
            if last == Some(x) {
                code.push('0');
                continue;
            } else {
                code.push('1');
                last = Some(x);
            }
        }
    
        // code = code.pad_left(7, '0');
        // pad left
        let mut ncode = "".to_string();
        for _ in 0..(7 - code.len()) {
            ncode.push('0');
        }
        code = "1".to_string() + &code;
        let code: i32 = i32::from_str_radix(&code, 2).unwrap();
        let code = format!("{:x}", code);
        code
    }


    fn reverse_code(code: &str) -> Result<String, &'static str> {
        // code is hex,convert to binary
        // check if code is valid
        // if startswith 0x, remove it
        let code = code.trim_start_matches("0x");
        

        let is_valid = code.chars().all(|c| c.is_digit(16));
        if !is_valid {
            return Err("invalid code provided, code must be hex")
        }
        // if code is valid, reverse it

        let code = i32::from_str_radix(&code, 16).unwrap();
        let code = format!("{:b}", code);

        // return "sdf".to_string();
        // let code: i32 = i32::from_str_radix(&code, 16).unwrap();
        let code = &code[1..];
        let mut ncode = code[0..1].to_string();

        for i in code[1..].chars() {
            if &ncode[ncode.len()-1..] == "0" && i == '0' {
                ncode.push('0');
                continue;
            }
            if &ncode[ncode.len()-1..] == "1" && i == '1' {
                ncode.push('0');
                continue;
            }
            if &ncode[ncode.len()-1..] == "1" && i == '0' {
                ncode.push('1');
                continue;
            }
            if &ncode[ncode.len()-1..] == "0" && i == '1' {
                ncode.push('1');
                continue;
            }
        }
        Ok(ncode)
    }



    pub fn get_roles_from_code(code: &str) -> Result<Vec<&'static str>, &'static str> {
        let bools = reverse_code(code);
        if !bools.is_ok() {
            // pass along err
            return Err(bools.unwrap_err());
        }
        let bools = bools.unwrap();
        
        let roles = get_roles();
        let mut tre = Vec::new();
        for (x, item) in roles.iter().enumerate() {
            if bools.chars().nth(x).unwrap() == '1' {
                tre.push(item.0);
            }
        }
        print!("{:?}", tre);
        Ok(tre)
    }
}
use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::fs::{self, File};
use std::io::{Read};

pub fn input(msg : &str) -> String{
    println!("{}", msg);
    let mut input_msg = String::new();
    io::stdin()
        .read_line(&mut input_msg)
        .expect("failed to read line");

    input_msg.trim().to_string()
}

pub fn visit_dirs(dir: &Path, exclude_dir: &Vec<&str>) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() && !exclude_dir.contains(&path.to_str().unwrap()) {
            let inner_vec = visit_dirs(&path, exclude_dir);
            for inner in inner_vec {
                vec.push(inner);
            }
        }
        else{
            let ext = match path.extension() {
                Some(ext) => ext.to_str().unwrap(),
                None => "",
            };
            if ext == "php" || ext == "html" {
                vec.push(path.to_str().unwrap().to_string());
            }
        }
    }
    vec
}

pub fn read_config(config_path: &str) -> HashMap<String, String>{
    let mut map = HashMap::new();
    let mut file = File::open(config_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        let mut items = line.split("=");
        let key = items.next().unwrap();
        let value = items.next().unwrap();
        map.insert(key.to_string(), value.to_string());
    }
    map
}

pub fn count_param(str: &str) -> usize {
    let mut count: usize = 0;
    let mut nest_count = 0;
    let mut single_quote_count = 0;
    let mut double_quote_count = 0;
    for c in str.chars() {
        match c {
            '(' => {
                nest_count += 1;
            },
            ')' => {
                nest_count -= 1;
            },
            '[' => {
                nest_count += 1;
            },
            ']' => {
                nest_count -= 1;
            },
            '\'' => {
                if single_quote_count > 0 {
                    single_quote_count -= 1;
                }
                else {
                    single_quote_count += 1;
                }

                if single_quote_count > 0 {
                    nest_count += 1;
                }
                else{
                    nest_count -= 1;
                }
            },
            '"' => {
                if double_quote_count > 0 {
                    double_quote_count -= 1;
                }
                else {
                    double_quote_count += 1;
                }

                if double_quote_count > 0 {
                    nest_count += 1;
                }
                else{
                    nest_count -= 1;
                }
            },
            ',' => {
                if nest_count == 0 {
                    count += 1;
                }
            },
            _ => {}
        }
    }
    count+1
}

pub fn find_n_param(str: &str, n: usize) -> String {
    let mut result = String::new();
    let mut count = 0;
    let mut nest_count = 0;
    let mut single_quote_count = 0;
    let mut double_quote_count = 0;
    for c in str.chars() {
        result.push(c);
        match c {
            '(' => {
                nest_count += 1;
            },
            ')' => {
                nest_count -= 1;
            },
            '[' => {
                nest_count += 1;
            },
            ']' => {
                nest_count -= 1;
            },
            '\'' => {
                if single_quote_count > 0 {
                    single_quote_count -= 1;
                }
                else {
                    single_quote_count += 1;
                }

                if single_quote_count > 0 {
                    nest_count += 1;
                }
                else{
                    nest_count -= 1;
                }
            },
            '"' => {
                if double_quote_count > 0 {
                    double_quote_count -= 1;
                }
                else {
                    double_quote_count += 1;
                }

                if double_quote_count > 0 {
                    nest_count += 1;
                }
                else{
                    nest_count -= 1;
                }
            },
            ',' => {
                if nest_count == 0 {
                    count += 1;

                    if count == n {
                        result.pop();
                        break;
                    }else{
                        result.clear();
                    }
                }
            },
            _ => {}
        }
    }
    result
}

pub fn replace_n_param<T>(str: &str, n: usize, mut replace_callback: T) -> String
    where T: FnMut(&str) -> String
{
    let count = count_param(str);
    let mut param_vec: Vec<String> = Vec::new();
    for i in 0..count {
        let param = find_n_param(str, i+1);
        if i+1 == n {
            param_vec.push(replace_callback(&param));
        }
        else{
            param_vec.push(param);
        }
        
    }
    param_vec.join(",")
}
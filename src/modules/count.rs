use pcre2::bytes::Regex;

use super::Module;
use super::Upgrade;
use super::transform;
use super::show_transform_result;
use super::replace_transform_result;

#[derive(Debug)]
pub struct Count{
    tracking_source: String,
    tracking_param_str: String,
    will_replace_params_index: Vec<usize>,
    module: Module
}

impl Count {

    pub fn new() -> Self{
        Self{
            tracking_source: String::new(),
            tracking_param_str: String::new(),
            will_replace_params_index: Vec::new(),
            module: Module{
                reg: Regex::new(r#"[^A-Za-z0-9_>:]{1}count\(((?:[^\(\)]+|\((?1)\))*)\)"#).unwrap(),
                replace_params_index: Some(vec![1]),
                exclude_params_index: None
            }
       }
    }
    

    fn parse_match(match_str: &str) -> bool{
        // 排除空参数情况，如 count() 或 function count()
        if match_str.trim().is_empty() {
            return false;
        }
        if match_str.contains("case when") {
            return false;
        }
        // 匹配两种情况：
        // 1. DISTINCT field_name 或 DISTINCT(field) 这种简单形式
        // 2. DISTINCT ' 或 DISTINCT " 这种 PHP 字符串拼接形式，如 DISTINCT ' . strtolower($var) . '_id'
        if Regex::new(r#"['\"]?(?i)distinct(?-i)\s*(?:\(?[a-z0-9\.]+\)?|['\"])"#).unwrap().is_match(match_str.as_bytes()).unwrap() {
            return false;
        }
        if Regex::new(r#"^[a-z_\.]+$"#).unwrap().is_match(match_str.as_bytes()).unwrap() {
            return false;
        }
        if Regex::new(r#"\s*\$[a-zA-z0-9_]+\s*=\s*[a-zA-z0-9_]+\s*"#).unwrap().is_match(match_str.as_bytes()).unwrap() {
            return false;
        }
        if match_str == "*" || match_str == "0" {
            return false;
        }
        if match_str.trim_start().starts_with("(array)") {
            return false;
        }
        true
    }

}

impl Upgrade for Count{
    
    fn view(&mut self, content: &String) -> bool {
        match transform(&self.module, content, Count::parse_match) {
            Some(v) => {
                self.tracking_source = v.0.0;
                self.tracking_param_str = v.0.1;
                self.will_replace_params_index = v.1;

                show_transform_result(&self.tracking_source, &self.tracking_param_str, &self.will_replace_params_index);
                true
            },
            None => false
        }
    }

    fn exec(&self, content: String) -> Result<String, String> {

        let result = replace_transform_result(content, &self.tracking_source, &self.tracking_param_str, &self.will_replace_params_index);
        Ok(result)
    }

}



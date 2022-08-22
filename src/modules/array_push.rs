use pcre2::bytes::Regex;

use super::Module;
use super::Upgrade;
use super::transform;
use super::show_transform_result;
use super::replace_transform_result;

#[derive(Debug)]
pub struct ArrayPush{
    tracking_source: String,
    tracking_param_str: String,
    will_replace_params_index: Vec<usize>,
    module: Module
}

impl ArrayPush {

    pub fn new() -> Self{
        Self{
            tracking_source: String::new(),
            tracking_param_str: String::new(),
            will_replace_params_index: Vec::new(),
            module: Module{
                reg: Regex::new(r#"[^A-Za-z0-9_]{1}array_push\(((?:[^\(\)]+|\((?1)\))*)\)"#).unwrap(),
                replace_params_index: Some(vec![1]),
                exclude_params_index: None
            }
       }
    }
    

    fn parse_match(match_str: &str) -> bool{
        if !match_str.trim_start().starts_with("(array)") {
            true
        }
        else{
            false
        }
    }

}

impl Upgrade for ArrayPush{
    
    fn view(&mut self, content: &String) -> bool {
        match transform(&self.module, content, ArrayPush::parse_match) {
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



use pcre2::bytes::Regex;

use super::Upgrade;
use crate::tools;

#[derive(Debug)]
pub struct ArrayDiff{
    reg: Regex
}

impl Upgrade for ArrayDiff{
    
    fn view(&self, content: &String) -> bool {
        match self.transform(content) {
            Some(v) => {
                println!("{}", v[0]);
                println!("↓");
                println!("{}", v[1]);
                true
            },
            None => false
        }
    }

    fn exec(&self, content: String) -> Result<String, String> {
        match self.transform(&content) {
            Some(v) => {
                let new_content = content.replace(v[0].as_str(), v[1].as_str());
                Ok(new_content)
            },
            None => Err("没有匹配项可处理".to_string())
        }
    }

}


impl ArrayDiff {

    pub fn new() -> Self{
        Self{
            reg: Regex::new(r#"[^A-Za-z0-9_]?array_diff\(((?:[^\(\)]+|\((?1)\))*)\)"#).unwrap()
        }
    }


    fn transform(&self, content: &String) -> Option<Vec<String>> {
        let mut content_copy = content.clone();
        while self.reg.is_match(content_copy.as_bytes()).unwrap() {
            
            let caps = self.reg.captures(content_copy.as_bytes()).unwrap().unwrap();
            let source = String::from_utf8(caps[0].to_vec()).unwrap();
            let group_1 = String::from_utf8(caps[1].to_vec()).unwrap();

            let param_count = tools::count_param(group_1.as_str());
            let mut target = group_1.clone();
            let mut flag = false;

            for i in 0..param_count {

                let param = tools::find_n_param(group_1.as_str(), i + 1);
                target = match self.transform_param(target.as_str(), param.as_str(), i+1) {
                    Some(v) => {
                        flag = true;
                        v
                    },
                    None => target
                };
            }
            
             match flag{
                true => {
                    target = source.replace(group_1.as_str(), target.as_str());
                    return Some(vec![source, target]);
                },
                false => {
                    content_copy = content_copy.replace(source.as_str(), "");
                }
            };

        }

        None
        
    }

    fn transform_param(&self, source: &str, param: &str, n: usize) -> Option<String>{
        if !param.trim_start().starts_with("(array)") {
            let result =  tools::replace_n_param(source, n, |x| format!("(array)({})", x).to_string());
            Some(result)
        }
        else {
            None
        }
    }

}
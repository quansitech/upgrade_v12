use pcre2::bytes::Regex;

use super::Upgrade;
use crate::tools;

#[derive(Debug)]
pub struct Implode{
    reg: Regex
}

impl Upgrade for Implode{
    
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


impl Implode {

    pub fn new() -> Self{
        Self{
            reg: Regex::new(r#"[^A-Za-z0-9_>]{1}implode\(((?:[^\(\)]+|\((?1)\))*)\)"#).unwrap()
        }
    }

    fn transform(&self, content: &String) -> Option<Vec<String>> {
        let mut content_copy = content.clone();
        while self.reg.is_match(content_copy.as_bytes()).unwrap() {
            
            let caps = self.reg.captures(content_copy.as_bytes()).unwrap().unwrap();
            println!("{:#?}", caps);
            let source = String::from_utf8(caps[0].to_vec()).unwrap();
            let group_1 = String::from_utf8(caps[1].to_vec()).unwrap();

            let param_2 = tools::find_n_param(group_1.as_str(), 2);

            match self.parse_match(param_2.as_str()) {
                true => {
                    let new_params = tools::replace_n_param(group_1.as_str(), 2, |x| format!("(array)({})", x).to_string());

                    let target = source.replace(group_1.as_str(), new_params.as_str());

                    return Some(vec![source, target]);
                },
                false => {
                    content_copy = content_copy.replace(source.as_str(), "");
                }
            };
        }
        None
    }

    fn parse_match(&self, match_str: &str) -> bool{
        if !match_str.trim_start().starts_with("(array)") {
            true
        }
        else{
            false
        }
    }
}
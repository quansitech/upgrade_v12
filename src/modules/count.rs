use pcre2::bytes::Regex;

use super::Upgrade;

#[derive(Debug)]
pub struct Count{
    reg: Regex
}

impl Upgrade for Count{
    
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


impl Count {

    pub fn new() -> Self{
        Count{
            reg: Regex::new(r#"[^A-Za-z0-9_]?count\(([^(?:\(array\))])((?:[^\(\)]+|\((?2)\))*)\)"#).unwrap()
        }
    }


    fn transform(&self, content: &String) -> Option<Vec<String>> {
        let mut content_copy = content.clone();
        while self.reg.is_match(content_copy.as_bytes()).unwrap() {
            let caps = self.reg.captures(content_copy.as_bytes()).unwrap().unwrap();
            let source = String::from_utf8(caps[0].to_vec()).unwrap();
            let group_1 = String::from_utf8(caps[1].to_vec()).unwrap();
            let group_2 = String::from_utf8(caps[2].to_vec()).unwrap();
            let match_str = format!("{}{}", group_1, group_2);
            match self.parse_match(match_str.as_str()) {
                true => {
                    let target = source.replace(format!("{}", match_str).as_str(), format!("(array)({})", match_str).as_str());

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
        if match_str.contains("case when") {
            return false;
        }
        if Regex::new(r#"['"]?(?i)distinct(?-i)\s*\(?[a-z0-9\.]+\)?['"]?"#).unwrap().is_match(match_str.as_bytes()).unwrap() {
            return false;
        }
        if Regex::new(r#"^[a-z_]+$"#).unwrap().is_match(match_str.as_bytes()).unwrap() {
            return false;
        }
        if Regex::new(r#"\s*\$[a-zA-z0-9_]+\s*=\s*[a-zA-z0-9_]+\s*"#).unwrap().is_match(match_str.as_bytes()).unwrap() {
            return false;
        }
        if match_str == "*" {
            return false;
        }
        true
    }
}
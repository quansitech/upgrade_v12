use pcre2::bytes::Regex;
use crate::tools;
use colored::*;

pub mod count;
pub mod array_column;
pub mod implode;
pub mod array_merge;
pub mod array_combine;
pub mod array_slice;
pub mod array_map;
pub mod array_diff;
pub mod array_filter;
pub mod array_keys;
pub mod array_intersect;
pub mod in_array;
pub mod array_chunk;
pub mod array_pop;
pub mod array_push;

pub trait Upgrade {

    fn view(&mut self, contents: &String) -> bool;
    fn exec(&self, contents: String) -> Result<String, String>;
}

#[derive(Debug)]
pub struct Module {
    pub reg: Regex,
    pub replace_params_index: Option<Vec<usize>>,
    pub exclude_params_index: Option<Vec<usize>>
}

fn get_code(reg: &Regex, content: &str) -> Option<(String, String)> {
    if reg.is_match(content.as_bytes()).unwrap() {
        let caps = reg.captures(content.as_bytes()).unwrap().unwrap();
        let source = String::from_utf8(caps[0].to_vec()).unwrap();
        let params_str = String::from_utf8(caps[1].to_vec()).unwrap();
        Some((source, params_str))
    } else {
        None
    }
}

fn transform<T>(module: &Module, content: &String, check_param: T) -> Option<((String, String), Vec<usize>)>
    where T: Fn(&str) -> bool {
    let mut content_copy = content.clone();
    let mut source = String::new();
    let mut params_str = String::new();

    while match get_code(&module.reg, content_copy.as_str()) {
        Some(v) => {
            source = v.0;
            params_str = v.1;
            true
        }
        None => false
    } {

        let param_count = tools::count_param(params_str.as_str());
        let mut can_replace_flag = false;
        let mut target_params_index: Vec<usize> = Vec::new();
        for i in 0..param_count {
            if module.replace_params_index != None && !module.replace_params_index.as_ref().unwrap().contains(&(i+1)) {
                continue;
            }

            if module.exclude_params_index != None && module.exclude_params_index.as_ref().unwrap().contains(&(i+1)) {
                continue;
            }

            let param = tools::find_n_param(params_str.as_str(), i + 1);
            if !check_param(param.as_str()) {
                continue;
            }

            can_replace_flag = true;
            target_params_index.push(i + 1);
        }

        if can_replace_flag {
            return Some(((source, params_str), target_params_index));
        } else {
            content_copy = content_copy.replace(source.as_str(), "");
        }
    }

    None
}

fn show_transform_result(source: &String, params_str: &String, target_param_index: &Vec<usize>) {
    let mut new_params_str = params_str.clone();
    let mut n = 0;
    let mut show_vec: Vec<String> = Vec::new();
    for i in target_param_index {
        new_params_str = tools::replace_n_param(new_params_str.as_str(), *i, |x| { 
            show_vec.push(format!("{}{}{}", "(array)(".red(), x, ")".red()).to_string());
            format!("#__{}__#", n.to_string()).to_string() 
        });
        n = n + 1;
    }

    for i in 0..n {
        new_params_str = new_params_str.replace(format!("#__{}__#", i.to_string()).as_str(), show_vec[i].as_str());
    }

    println!("{}", source);
    println!("↓");
    println!("{}", source.replace(params_str, new_params_str.as_str()));
}

fn replace_transform_result(content: String, source: &String, params_str: &String, target_param_index: &Vec<usize>) -> String {
    let mut new_params_str = params_str.clone();
    for i in target_param_index {
        new_params_str = tools::replace_n_param(new_params_str.as_str(), *i, |x| format!("(array)({})", x).to_string())
    }

    content.replace(source.as_str(), source.replace(params_str, new_params_str.as_str()).as_str())
}
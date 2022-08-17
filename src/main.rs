use std::path::Path;
use std::env;

mod modules;
mod file_handle;
mod tools;

use crate::modules::count::Count;
use crate::modules::array_column::ArrayColumn;
use crate::modules::implode::Implode;
use crate::modules::array_merge::ArrayMerge;
use crate::modules::array_combine::ArrayCombine;
use crate::modules::array_slice::ArraySlice;
use crate::modules::array_map::ArrayMap;
use crate::modules::array_diff::ArrayDiff;
use crate::modules::array_filter::ArrayFilter;
use crate::modules::array_keys::ArrayKeys;
use crate::modules::array_intersect::ArrayIntersect;
use crate::modules::in_array::InArray;
use crate::file_handle::FileUpgrade;

fn main() -> Result<(), String> {

    let args: Vec<String> = env::args().collect();

    let config_path = &args[1];

    let config_map = tools::read_config(config_path.as_str());

    let path_str = config_map.get("path").unwrap();

    let path = Path::new(path_str);

    let tmp_exclude_dir: Vec<String>= config_map.get("exclude_dir").unwrap().split(",").map(|x| -> String {
        let new_str = format!("{}/{}", path_str, x);
        new_str
    }).collect();

    let exclude_dir: Vec<&str> = tmp_exclude_dir.iter().map(|x| x.as_str()).collect();

    

    let vec: Vec<String> = tools::visit_dirs(&path, &exclude_dir);

    // vec = vec!("/mnt/www/move/demo.php".to_string());

    let mut file_upgrade = FileUpgrade::new();

    let count = Box::new(Count::new());
    let array_column = Box::new(ArrayColumn::new());
    let implode = Box::new(Implode::new());
    let array_merge = Box::new(ArrayMerge::new());
    let array_combine = Box::new(ArrayCombine::new());
    let array_slice = Box::new(ArraySlice::new());
    let array_map = Box::new(ArrayMap::new());
    let array_diff = Box::new(ArrayDiff::new());
    let array_filter = Box::new(ArrayFilter::new());
    let array_keys = Box::new(ArrayKeys::new());
    let array_intersect = Box::new(ArrayIntersect::new());
    let in_array = Box::new(InArray::new());


    file_upgrade.register_upgrade_module(count);
    file_upgrade.register_upgrade_module(array_column);
    file_upgrade.register_upgrade_module(implode);
    file_upgrade.register_upgrade_module(array_merge);
    file_upgrade.register_upgrade_module(array_combine);
    file_upgrade.register_upgrade_module(array_slice);
    file_upgrade.register_upgrade_module(array_map);
    file_upgrade.register_upgrade_module(array_diff);
    file_upgrade.register_upgrade_module(array_filter);
    file_upgrade.register_upgrade_module(array_keys);
    file_upgrade.register_upgrade_module(array_intersect);
    file_upgrade.register_upgrade_module(in_array);

    for index in 0..vec.len(){
        println!("正在处理文件: {}", vec[index]);

        file_upgrade.set_file_path(vec[index].as_str());

        file_upgrade.upgrade()?;

        println!();
    }
    
    Ok(())

    // if re.is_match(&contents) {
    //     let caps = re.captures(&contents).unwrap();
    //     let count_content = caps.get(0).unwrap().as_str();
    //     println!("{:#?}", caps);
    // }

    
}



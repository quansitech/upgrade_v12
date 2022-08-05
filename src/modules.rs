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

pub trait Upgrade {
    fn view(&self, contents: &String) -> bool;
    fn exec(&self, contents: String) -> Result<String, String>;
}
use std::fs::OpenOptions;
use std::io::{Read, Seek, Write};
use crate::modules::Upgrade;
use crate::tools;


pub struct FileUpgrade<'a> {
    file_path: &'a str,
    upgrade_modules: Vec<Box<dyn Upgrade>>
}

impl<'a> FileUpgrade<'a>{

    pub fn new() -> Self {
        Self {
            file_path: "",
            upgrade_modules: Vec::new()
        }
    }

    pub fn set_file_path(&mut self, file_path: &'a str) {
        self.file_path = file_path;
    }

    pub fn register_upgrade_module(&mut self, upgrade_module: Box<dyn Upgrade>) {
        self.upgrade_modules.push(upgrade_module);
    }

    pub fn upgrade(&self) -> Result<(), String> {

        let length = self.upgrade_modules.len();
        let mut file_upgrade_modules: Vec<usize> = (0..length).collect();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.file_path).unwrap();

        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        while !file_upgrade_modules.is_empty() {
            let mut tmp_upgrade_modules: Vec<usize> = Vec::new();
            for index in file_upgrade_modules {
                match self.upgrade_modules[index].view(&contents) {
                    true => {
                        let cmd = tools::input("输入回车进行替换操作，输入q退出");
                        match cmd.as_str() {
                            "q" => return Err("退出".to_string()),
                            _ => {
                                contents = self.upgrade_modules[index].exec(contents).unwrap();
                            }
                        }
                        tmp_upgrade_modules.push(index);
                    }
                    _ => {}
                }
            }
            file_upgrade_modules = tmp_upgrade_modules;
        }

        file.rewind().unwrap();

        file.write_all(contents.as_bytes()).unwrap();

        file.sync_all().unwrap();

        Ok(())
    }
}
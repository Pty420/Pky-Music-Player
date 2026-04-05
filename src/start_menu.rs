use crate::utilitys::system_fn::system;
use std::io;
pub fn start_menu() -> String {
    system("clear");
    println!(
        "
-------------PkyMusic-------------
|                                |
|         请选择读取模式         |
|            ↓     ↓             |
|          Fold   File           |
|          文件夹 文件           |
|                                |
----------------------------------"
    );
    println!("fold/file:");
    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        if choice == "fold" {
            println!("功能待完善");
            continue;
        } else if choice == "file" {
            return "file".to_string();
        } else {
            println!("\"{}\"是无效指令! 请重新输入:", choice);
            continue;
        }
    }
}

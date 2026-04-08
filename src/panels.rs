use crate::utilitys::system_fn::system;
use rodio::Sink;
pub fn start_menu() -> String {
    system("clear");
    println!(
        "
---------------PkyMusic---------------
|                                    |
|           请选择读取模式           |
|              ↓     ↓               |
|            Fold   File             |
|            文件夹 文件             |
|                                    |
--------------------------------------"
    );
    println!("fold/file:");
    loop {
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
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

pub fn mode_selected(){
    println!(
        "
-------------PkyMusic--------------
|                                 |
|         mode  ->  file          |
|                                 |
-----------------------------------"
    );
}
pub fn state_panel(sink:&Sink){
    println!(
        "
---------------PkyMusic----------------
 ♫     ♫     ♫     ♫     ♫     ♫     ♫
---------------------------------------
 音量:{:.2} 状态:{}
---------------------------------------
 ♪   ♪   ♪   ♪   ♪   ♪   ♪   ♪   ♪   ♪
---------------------------------------
",
    sink.volume(),if sink.is_paused() {"暂停中"}else if !sink.is_paused() {"播放中"}else {"未知状态"}
    );
}

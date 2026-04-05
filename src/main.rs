use crate::file_mode::BackPathMode;
use file_mode::play_cmd;
use rodio::{Decoder, OutputStream, Sink, Source};
use start_menu::start_menu;
use std::fs::File;
use utilitys::system_fn::system;
mod file_mode;
mod start_menu;
mod utilitys;
fn main() {
    loop {
        let read_mode = start_menu();
        system("clear");
        if read_mode == "file" {
            println!(
                "
------------PkyMusic------------
|                              |
|         mode -> file         |
|                              |
--------------------------------"
            );
            let BackPathMode(path, is_loop) = play_cmd();
            let (_stream, stream_controller) = OutputStream::try_default().unwrap();
            let audio_file = File::open(path).unwrap();
            let controller = Sink::try_new(&stream_controller).unwrap();
            let audio_src = Decoder::new(audio_file).unwrap();
            controller.set_volume(0.3);
            if is_loop {
                controller.append(audio_src.repeat_infinite());
                controller.play();
                println!("正在循环播放指定音乐...");
                while !controller.empty() {
                    let mut get_input = String::new();
                    std::io::stdin().read_line(&mut get_input).unwrap();
                    let c: char = get_input.trim().parse().unwrap();
                    if c == 'p' {
                        controller.pause();
                    } else if c == 'c' {
                        controller.play();
                    } else if c == 'q' {
                        break;
                    } else {
                        println!("命令无效");
                        continue;
                    }
                }
            } else {
                controller.append(audio_src);
                controller.play();
                println!("正在播放一次指定音乐...");
                while !controller.empty() {
                    let mut get_input = String::new();
                    std::io::stdin().read_line(&mut get_input).unwrap();
                    let c: char = get_input.trim().parse().unwrap();
                    if c == 'p' {
                        controller.pause();
                    } else if c == 'c' {
                        controller.play();
                    } else if c == 'q' {
                        break;
                    } else {
                        println!("命令无效");
                        continue;
                    }
                }
                println!("播放结束");
                system("sleep 2");
            }
        }
    }
}

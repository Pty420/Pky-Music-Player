use crate::file_mode::BackPathMode;
use file_mode::{play_cmd,file_playing_cmd};
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
                    file_playing_cmd(&controller);
                }
                println!("播放结束");
                system("sleep 2");
            } else {
                controller.append(audio_src);
                controller.play();
                println!("正在播放一次指定音乐...");
                while !controller.empty() {
                    file_playing_cmd(&controller);
                }
                println!("播放结束");
                system("sleep 2");
            }
        }
    }
}

use crate::file_mode::BackPathMode;
use file_mode::{play_cmd,file_playing_cmd};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use panels::{start_menu,mode_selected,state_panel};
use clearscreen::clear;
use std::thread;
use std::time;
mod file_mode;
mod panels;
mod utilitys;
fn main() {
    loop {
        clear().unwrap();
        let read_mode = start_menu();
        clear().unwrap();
        mode_selected();
        if read_mode == "file" {
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
                    clear().unwrap();
                    state_panel(&controller);
                    file_playing_cmd(&controller);
                }
                println!("播放结束");
                thread::sleep(time::Duration::from_secs(2));
            } else {
                controller.append(audio_src);
                controller.play();
                println!("正在播放一次指定音乐...");
                while !controller.empty() {
                    clear().unwrap();
                    state_panel(&controller);
                    file_playing_cmd(&controller);
                }
                println!("播放结束");
                thread::sleep(time::Duration::from_secs(2));
            }
        }
    }
}

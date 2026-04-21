use clearscreen::clear;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use panels::state_panel;
use play_cmd::BackPathMode;
use play_cmd::play_cmd;
use playing::playing_cmd;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::{fs::File, thread, time};
use utilitys::append_audio_files::append_audio_files;
mod panels;
mod play_cmd;
mod playing;
mod utilitys;
fn main() {
    loop {
        let BackPathMode(path, is_loop) = play_cmd();
        let (_stream, stream_controller) = OutputStream::try_default().unwrap();
        if path.is_file() {
            let controller = Sink::try_new(&stream_controller).unwrap();
            controller.set_volume(0.3);
            if is_loop {
                let audio_file = File::open(&path).expect("文件无法读取!");
                let audio_src = Decoder::new(audio_file).expect("文件解析失败!");
                controller.append(audio_src.repeat_infinite());
                controller.play();
                println!("正在循环播放指定音乐...");
                enable_raw_mode().expect("终端模式切换失败!");
                while !controller.empty() {
                    clear().unwrap();
                    state_panel(&controller, &path, 1);
                    playing_cmd(&controller);
                }
                disable_raw_mode().expect("终端模式切换失败!");
                println!("播放结束");
                thread::sleep(time::Duration::from_secs(2));
            } else {
                append_audio_files(&path, &controller);
                controller.play();
                println!("正在播放一次指定音乐...");
                enable_raw_mode().expect("终端模式切换失败!");
                while !controller.empty() {
                    clear().unwrap();
                    state_panel(&controller, &path, 1);
                    playing_cmd(&controller);
                }
                disable_raw_mode().expect("终端模式切换失败!");
                println!("播放结束");
                thread::sleep(time::Duration::from_secs(2));
            }
        } else {
            let controller = Sink::try_new(&stream_controller).unwrap();
            let mut is_quit = false;
            while !is_quit {
                append_audio_files(&path, &controller);
                let files_count = controller.len();
                controller.set_volume(0.3);
                enable_raw_mode().expect("终端模式切换失败!");
                while !controller.empty() {
                    clear().unwrap();
                    state_panel(&controller, &path, files_count);
                    is_quit = playing_cmd(&controller);
                }
                disable_raw_mode().expect("终端模式切换失败!");
            }
            println!("播放结束");
            thread::sleep(time::Duration::from_secs(2));
        }
    }
}

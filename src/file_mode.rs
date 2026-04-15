use crate::panels::mode_selected;
use crate::utilitys::check_char::check_head_char;
use clearscreen::clear;
use dirs::audio_dir;
use rodio::Sink;
use std::{path::PathBuf, thread, time};
pub struct BackPathMode(pub PathBuf, pub bool);
pub fn play_cmd() -> BackPathMode {
    loop {
        clear().unwrap();
        mode_selected();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("未知错误！");
        let line = line.trim();
        let mut s_line = line.split_whitespace();
        match s_line.next() {
            Some("play") => {
                let mut full_path = PathBuf::new();
                let mut line_cp = s_line.clone();
                let is_loop = match line_cp.next() {
                    Some("-l") => {
                        let mut path = PathBuf::new();
                        let mut path_str = String::new();
                        loop {
                            match line_cp.next() {
                                Some(str) => {
                                    if !path_str.is_empty() {
                                        path_str.push_str(" ");
                                    }
                                    path_str = path_str + &str;
                                    continue;
                                }
                                None => {
                                    path.push(path_str);
                                    break;
                                }
                            }
                        }
                        if check_head_char(&path.to_string_lossy(), '/') {
                            full_path.push(path);
                        } else {
                            full_path =
                                PathBuf::from(audio_dir().unwrap().to_string_lossy().to_string());
                            full_path.push(path);
                        }
                        true
                    }
                    Some(other) => {
                        if check_head_char(other, '-') {
                            println!("无效参数\"{}\"", other);
                            thread::sleep(time::Duration::from_secs(1));
                            continue;
                        } else {
                            let mut path = PathBuf::new();
                            let mut path_str = String::new();
                            loop {
                                match s_line.next() {
                                    Some(str) => {
                                        if !path_str.is_empty() {
                                            path_str.push_str(" ");
                                        }
                                        path_str = path_str + &str;
                                        continue;
                                    }
                                    None => {
                                        path.push(path_str);
                                        break;
                                    }
                                }
                            }
                            if check_head_char(other, '/') {
                                full_path.push(path);
                            } else {
                                full_path = PathBuf::from(
                                    audio_dir().unwrap().to_string_lossy().to_string(),
                                );
                                full_path.push(path);
                            }
                        }
                        false
                    }
                    None => {
                        println!("缺少有效路径！");
                        thread::sleep(time::Duration::from_secs(1));
                        continue;
                    }
                };
                return BackPathMode(full_path, is_loop);
            }
            Some(other) => {
                println!("无效参数\"{}\"", other);
                thread::sleep(time::Duration::from_secs(1));
                continue;
            }
            None => {
                continue;
            }
        }
    }
}
pub fn file_playing_cmd(sink: &Sink) {
    let mut get_line = String::new();
    std::io::stdin()
        .read_line(&mut get_line)
        .expect("无效指令！");
    let get_line = get_line.trim();
    let mut s_line = get_line.split_whitespace();
    match s_line.next() {
        Some("pause") => sink.pause(),
        Some("play") => sink.play(),
        Some("quit") => sink.clear(),
        Some("set") => match s_line.next() {
            Some("volume") => {
                if let Ok(volume) = s_line.next().unwrap().trim().parse::<f32>() {
                    if let None = s_line.next() {
                        if volume >= 0.0 && volume <= 1.0 {
                            sink.set_volume(volume);
                        } else {
                            println!("音量大小必须在 0.0 ~ 1.0 之间！");
                            thread::sleep(time::Duration::from_secs(1));
                        }
                    } else {
                        println!("存在未知指令或参数!");
                        thread::sleep(time::Duration::from_secs(1));
                    }
                } else {
                    println!("音量大小必须是一个有效数字！");
                    thread::sleep(time::Duration::from_secs(1));
                }
            }
            Some(other) => {
                println!("未能匹配到指令\"{}\"", other);
                thread::sleep(time::Duration::from_secs(1));
            }
            None => {
                println!("没有指定音量大小！");
                thread::sleep(time::Duration::from_secs(1));
            }
        },
        Some(other) => {
            println!("未能匹配到指令\"{}\"", other);
            thread::sleep(time::Duration::from_secs(1));
        }
        None => {
            println!("命令不能为空！");
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}

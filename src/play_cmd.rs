use crate::panels::start_menu;
use crate::utilitys::check_char::check_head_char;
use clearscreen::clear;
use dirs::audio_dir;
use std::{path::PathBuf, thread, time};
pub struct BackPathMode(pub PathBuf, pub bool);
pub fn play_cmd() -> BackPathMode {
    loop {
        clear().unwrap();
        start_menu();
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
            Some("open") => {
                let mut path_str = String::new();
                let mut line_cp = s_line.clone();
                let mut path = PathBuf::new();
                match line_cp.next() {
                    Some(head) => {
                        path_str = path_str + &head;
                        loop {
                            match line_cp.next() {
                                Some(str) => {
                                    path_str.push_str(" ");
                                    path_str = path_str + &str;
                                    continue;
                                }
                                None => {
                                    path.push(path_str);
                                    break;
                                }
                            }
                        }
                    }
                    None => {
                        println!("缺少有效路径！");
                        continue;
                    }
                }
                if check_head_char(&path.to_string_lossy(), '/') {
                    return BackPathMode(path, false);
                } else {
                    let mut full_path = PathBuf::from(audio_dir().unwrap());
                    full_path.push(path);
                    return BackPathMode(full_path, false);
                }
            }
            Some(other) => {
                println!("无效指令\"{}\"", other);
                thread::sleep(time::Duration::from_secs(1));
                continue;
            }
            None => {
                continue;
            }
        }
    }
}

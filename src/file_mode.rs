use crate::utilitys::check_char::check_head_char;
use dirs::audio_dir;
use rodio::Sink;
pub struct BackPathMode(pub String, pub bool);
pub fn play_cmd() -> BackPathMode {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("无效指令！");
        let line = line.trim();
        let mut s_line = line.split_whitespace();
        if let Some(play) = s_line.next() {
            if play == "play" {
                let mut path = String::new();
                let is_loop = match s_line.next() {
                    Some("-l") => true,
                    Some(other) => {
                        if check_head_char(other, '-') {
                            println!("无效参数\"{}\"", other);
                            continue;
                        } else {
                            let mut path_part = vec![other];
                            path_part.extend(s_line.clone());
                            path = path_part.join(" ");
                            false
                        }
                    }
                    None => {
                        println!("缺少有效路径！");
                        continue;
                    }
                };
                let base_path = audio_dir().unwrap().to_string_lossy().to_string();
                if is_loop {
                    let mut path_part: Vec<&str> = vec![s_line.next().unwrap()];
                    path_part.extend(s_line.clone());
                    let full = path_part.join(" ");
                    if let Some(path_next3) = Some(full) {
                        if check_head_char(&path_next3, '/') {
                            return BackPathMode(path_next3.to_string(), is_loop);
                        } else {
                            return BackPathMode(
                                (base_path + "/" + &path_next3).to_string(),
                                is_loop,
                            );
                        }
                    } else {
                        println!("缺少有效路径！");
                        continue;
                    }
                } else {
                    if check_head_char(&path, '/') {
                        return BackPathMode(path.to_string(), is_loop);
                    } else {
                        return BackPathMode((base_path + "/" + &path).to_string(), is_loop);
                    }
                }
            } else {
                println!("未能匹配到指令\"{}\"", play);
                continue;
            }
        } else {
            continue;
        }
    }
}
pub fn file_playing_cmd(sink: &Sink) {
    let mut get_line = String::new();
    std::io::stdin().read_line(&mut get_line).expect("无效指令！");
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
                        if volume>=0.0 && volume<=1.0 {
                            sink.set_volume(volume);
                        } else {
                            println!("音量大小必须在 0.0 ~ 1.0 之间！");
                        }
                    } else {
                        println!("存在未知指令或参数!");
                    }
                } else {
                    println!("音量大小必须是一个有效数字！");
                }
            },
            Some(other) => println!("未能匹配到指令\"{}\"", other),
            None => println!("没有指定音量大小！"),
        },
        Some(other) => println!("未能匹配到指令\"{}\"",other),
        None => println!("命令不能为空！"),
    }
}

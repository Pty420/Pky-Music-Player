use rodio::Sink;
use std::{thread, time};
pub fn playing_cmd(sink: &Sink) -> bool {
    let mut get_line = String::new();
    std::io::stdin()
        .read_line(&mut get_line)
        .expect("无效指令！");
    let get_line = get_line.trim();
    let mut s_line = get_line.split_whitespace();
    match s_line.next() {
        Some("pause") => sink.pause(),
        Some("play") => sink.play(),
        Some("skip") => {
            if sink.len() > 1 {
                sink.skip_one();
            } else {
                sink.clear();
            }
        }
        Some("quit") => {
            sink.clear();
            return true;
        }
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
    false
}

use crossterm::event::{Event, KeyCode, poll, read};
use rodio::Sink;
use std::time::Duration;
pub fn playing_cmd(sink: &Sink) -> bool {
    if poll(Duration::from_millis(100)).expect("监听键盘输入出现错误!") {
        if let Event::Key(key) = read().expect("获取按键出现错误!") {
            match key.code {
                KeyCode::Char(' ') => {
                    if sink.is_paused() {
                        sink.play();
                    } else {
                        sink.pause();
                    }
                }
                KeyCode::Up => {
                    sink.set_volume(sink.volume() + 0.05);
                }
                KeyCode::Down => {
                    sink.set_volume(sink.volume() - 0.05);
                }
                KeyCode::Esc => {
                    sink.clear();
                    return true;
                }
                KeyCode::Char('n') => {
                    sink.skip_one();
                }
                _ => {}
            }
        }
    }
    false
}

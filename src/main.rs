use crossterm::{
    cursor,
    event::{Event, KeyCode, poll, read},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use dirs::audio_dir;
use panels::{draw_file_screen, draw_menu_screen, draw_playing_screen};
use play::play_file;
use ratatui::{Terminal, backend::CrosstermBackend, widgets::ListState};
use rodio::{OutputStream, Sink};
use std::{io::stdout, time};
use utilitys::{
    append_audio_files::append_audio_files,
    check_fold::{file_names, files_count},
};
mod panels;
mod play;
mod utilitys;
fn main() {
    execute!(stdout(), EnterAlternateScreen).unwrap();
    enable_raw_mode().unwrap();
    execute!(stdout(), cursor::Hide).unwrap();
    let (_stream, stream_controller) = OutputStream::try_default().expect("无法获取默认输出设备!");
    let sink = Sink::try_new(&stream_controller).expect("创建音频控制器失败!");
    sink.set_volume(0.3);
    let mut player_terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    let mut menu_select = ListState::default();
    menu_select.select(Some(0));
    let mut path_select = ListState::default();
    path_select.select(Some(0));
    let mut window_number = 0;
    let mut path = audio_dir().unwrap().to_path_buf();
    let mut audio_files: Vec<String> = Vec::new();
    let mut audio_number: usize = 0;
    loop {
        player_terminal
            .draw(|frame| match window_number {
                0 => draw_menu_screen(frame, &mut menu_select),
                1 => draw_file_screen(frame, &path, &mut path_select),
                2 => draw_file_screen(frame, &path, &mut path_select),
                3 => {
                    let mut full_path = path.clone();
                    full_path.push(&audio_files[audio_number]);
                    draw_playing_screen(frame, &audio_files[audio_number], &sink, &path);
                    play_file(&sink, &full_path);
                }
                _ => {}
            })
            .unwrap();
        if poll(time::Duration::from_millis(100)).expect("监听键盘输入出现错误!") {
            if let Event::Key(key) = read().expect("获取按键信息出现错误!") {
                match key.code {
                    KeyCode::Esc => match window_number {
                        0 => break,
                        1 => window_number = 0,
                        2 => window_number = 0,
                        3 => {
                            window_number = 0;
                            sink.clear();
                            audio_files.clear();
                            audio_number = 0;
                        }
                        _ => {}
                    },
                    KeyCode::Up => match window_number {
                        0 => {
                            let number = menu_select.selected().unwrap_or(0);
                            if number != 0 {
                                menu_select.select(Some(number - 1));
                            }
                        }
                        1 => {
                            let number = path_select.selected().unwrap_or(0);
                            if number != 0 {
                                path_select.select(Some(number - 1));
                            }
                        }
                        2 => {
                            let number = path_select.selected().unwrap_or(0);
                            if number != 0 {
                                path_select.select(Some(number - 1));
                            }
                        }
                        3 => {
                            sink.set_volume(sink.volume() + 0.05);
                        }
                        _ => {}
                    },
                    KeyCode::Down => match window_number {
                        0 => {
                            let number = menu_select.selected().unwrap_or(0);
                            if number != 1 {
                                menu_select.select(Some(number + 1));
                            }
                        }
                        1 => {
                            let number = path_select.selected().unwrap_or(0);
                            if number != files_count(&path) {
                                path_select.select(Some(number + 1));
                            }
                        }
                        2 => {
                            let number = path_select.selected().unwrap_or(0);
                            if number != files_count(&path) {
                                path_select.select(Some(number + 1));
                            }
                        }
                        3 => {
                            sink.set_volume(sink.volume() - 0.05);
                        }
                        _ => {}
                    },
                    KeyCode::PageDown => {
                        if window_number == 3 {
                            sink.clear();
                            sink.play();
                            audio_number = if audio_number < audio_files.len() - 1 {
                                audio_number + 1
                            } else {
                                0
                            }
                        }
                    }
                    KeyCode::PageUp => {
                        if window_number == 3 {
                            sink.clear();
                            sink.play();
                            audio_number = if audio_number > 0 {
                                audio_number - 1
                            } else {
                                audio_files.len() - 1
                            }
                        }
                    }
                    KeyCode::Enter => match window_number {
                        0 => {
                            let number = menu_select.selected().expect("选中未知选项!");
                            match number {
                                0 => window_number = 1,
                                1 => window_number = 2,
                                _ => {}
                            }
                        }
                        1 => {
                            let number = path_select.selected().expect("选中未知文件!");
                            let list = file_names(&path);
                            let mut full_path = path.clone();
                            full_path.push(&list[number]);
                            if full_path.is_dir() {
                                path = full_path;
                            } else {
                                let extension = full_path.extension().expect("文件类型判断失败!");
                                if extension == "mp3"
                                    || extension == "flac"
                                    || extension == "wav"
                                    || extension == "ogg"
                                {
                                    window_number = 3;
                                    audio_files.push(
                                        full_path
                                            .file_name()
                                            .expect("文件名称异常!")
                                            .to_string_lossy()
                                            .to_string(),
                                    );
                                }
                            }
                        }
                        2 => {
                            let number = path_select.selected().expect("选中未知文件!");
                            let list = file_names(&path);
                            let mut full_path = path.clone();
                            full_path.push(&list[number]);
                            if full_path.is_dir() {
                                path = full_path;
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Backspace => {
                        if window_number == 1 {
                            path.pop();
                        } else if window_number == 2 {
                            path.pop();
                        }
                    }
                    KeyCode::Char(' ') => {
                        if window_number == 3 {
                            if sink.is_paused() {
                                sink.play();
                            } else {
                                sink.pause();
                            }
                        }
                    }
                    KeyCode::Char('o') => {
                        if window_number == 2 {
                            let number = path_select.selected().expect("选中未知文件!");
                            let list = file_names(&path);
                            let mut full_path = path.clone();
                            full_path.push(&list[number]);
                            if full_path.is_dir() {
                                path = full_path;
                                audio_files = append_audio_files(&path);
                                window_number = 3;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    execute!(stdout(), cursor::Show).unwrap();
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen).unwrap();
}

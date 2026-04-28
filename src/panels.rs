use crate::utilitys::check_fold::check_fold;
use ratatui::{
    prelude::{Constraint, Direction, Frame, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};
use rodio::Sink;
use std::path::PathBuf;
pub fn draw_menu_screen(frame: &mut Frame, menu_select: &mut ListState) {
    let terminal_area = frame.area();
    let main_window = Block::default()
        .title(Line::from("Pky Music Player").centered())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    frame.render_widget(&main_window, terminal_area);
    let menu = List::new(vec![
        ListItem::new(Line::from("打开文件").centered()),
        ListItem::new(Line::from("打开文件夹").centered()),
    ])
    .style(Style::default().fg(Color::White))
    .block(Block::default().padding(Padding::top(2)))
    .highlight_style(Style::default().fg(Color::Black).bg(Color::Green));
    frame.render_stateful_widget(menu, main_window.inner(terminal_area), menu_select);
}
pub fn draw_file_screen(frame: &mut Frame, path: &PathBuf, path_select: &mut ListState) {
    let terminal_area = frame.area();
    let main_window = Block::default()
        .title(Line::from("Pky Music Player").centered())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    frame.render_widget(&main_window, terminal_area);
    let dir_list = check_fold(path)
        .style(Style::default().fg(Color::White))
        .block(Block::default().padding(Padding::top(1)))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Green));
    frame.render_stateful_widget(dir_list, main_window.inner(terminal_area), path_select);
}
pub fn draw_playing_screen(frame: &mut Frame, name: &str, sink: &Sink, path: &PathBuf) {
    let terminal_area = frame.area();
    let main_window = Block::default()
        .title(Line::from("Pky Music Player").centered())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    frame.render_widget(&main_window, terminal_area);
    let child_window = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(main_window.inner(terminal_area));
    let format_name = if sink.empty() {
        format!(" 空")
    } else {
        format!(
            "\n 音频文件名称:{}  所在文件夹:{}",
            name,
            path.to_string_lossy().to_string()
        )
    };
    let audio_name = Paragraph::new(format_name).style(Style::default().fg(Color::Green));
    frame.render_widget(audio_name, child_window[0]);
    let playing_or_pause = if sink.is_paused() {
        String::from("暂停中")
    } else {
        String::from("播放中")
    };
    let volume = sink.volume();
    let format_state = format!(
        "\n 音量:{:.0} 播放状态:{}",
        volume * 100.0,
        playing_or_pause
    );
    let state = Paragraph::new(format_state).style(Style::default().fg(Color::Blue));
    frame.render_widget(state, child_window[1]);
}

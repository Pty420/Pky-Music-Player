use ratatui::{
    text::Line,
    widgets::{List, ListItem},
};
use std::{fs::read_dir, path::PathBuf};
pub fn check_fold(path: &PathBuf) -> List<'static> {
    let mut file_list: Vec<ListItem> = Vec::new();
    for i in read_dir(&path).expect("读取目录失败!") {
        let f = ListItem::new(
            Line::from(
                i.expect("读取文件条目失败")
                    .path()
                    .file_name()
                    .expect("文件名称异常!")
                    .to_string_lossy()
                    .to_string(),
            )
            .centered(),
        );
        file_list.push(f);
    }
    let list = List::new(file_list);
    list
}
pub fn files_count(path: &PathBuf) -> usize {
    let mut count = 0;
    for _ in read_dir(&path).expect("读取目录失败!") {
        count += 1;
    }
    count
}
pub fn file_names(path: &PathBuf) -> Vec<String> {
    let mut name: Vec<String> = Vec::new();
    for i in read_dir(&path).expect("读取目录失败!") {
        let f = i
            .expect("读取文件条目失败")
            .path()
            .file_name()
            .expect("文件名称异常!")
            .to_string_lossy()
            .to_string();
        name.push(f);
    }
    name
}

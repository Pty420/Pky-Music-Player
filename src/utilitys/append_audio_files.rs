use std::{fs::read_dir, path::Path};
pub fn append_audio_files(path: &Path) -> Vec<String> {
    let mut audio_file_list: Vec<String> = Vec::new();
    for i in read_dir(&path).expect("目录打开失败！") {
        let f = i.expect("读取文件条目失败！").path();
        if f.is_file() {
            if let Some(file_type) = f.extension() {
                if file_type == "mp3"
                    || file_type == "flac"
                    || file_type == "wav"
                    || file_type == "ogg"
                {
                    audio_file_list.push(
                        f.file_name()
                            .expect("文件名称异常!")
                            .to_string_lossy()
                            .to_string(),
                    );
                }
            }
        }
    }
    audio_file_list
}

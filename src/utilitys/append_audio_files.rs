use rodio::{Decoder, Sink};
use std::{
    fs::{File, read_dir},
    path::Path,
};
pub fn append_audio_files(path: &Path, sink: &Sink) {
    if path.is_file() {
        let audio_file = File::open(path).expect("文件无法读取!");
        let audio_src = Decoder::new(audio_file).expect("文件解析失败!");
        sink.append(audio_src);
    } else {
        for i in read_dir(&path).expect("目录打开失败！") {
            let f = i.expect("读取文件条目失败！").path();
            let i = f.to_string_lossy().to_string();
            if f.is_file() {
                if let Some(file_type) = f.extension() {
                    if file_type == "mp3"
                        || file_type == "flac"
                        || file_type == "wav"
                        || file_type == "ogg"
                    {
                        let audio_file = File::open(i).expect("文件无法读取!");
                        let audio_src = Decoder::new(audio_file).expect("文件解析失败！");
                        sink.append(audio_src);
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        }
    }
}

use rodio::{Decoder, Sink};
use std::{fs::File, path::Path};
pub fn play_file(sink: &Sink, path: &Path) {
    let audio_file = File::open(path).expect("文件无法打开!");
    let audio_src = Decoder::new(audio_file).expect("文件解析失败!");
    sink.append(audio_src);
}

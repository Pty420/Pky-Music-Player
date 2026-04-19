use rodio::Sink;
use std::path::Path;
pub fn start_menu() {
    println!(
        "
-------------------------PkyMusic-------------------------
|                                                        |
|                     欢迎使用PkyMusic                   |
|                                                        |
|                      请输入您的指令                    |
|                                                        |
|             支持识别文件夹文件和直接播放文件           |
|                                                        |
----------------------------------------------------------"
    );
}
pub fn state_panel(sink: &Sink, path: &Path, files_count: usize) {
    let audio_name = path.file_stem().unwrap().to_string_lossy();
    println!(
        "
---------------------PkyMusic----------------------
 ♫     ♫     ♫     ♫     ♫     ♫     ♫     ♫     ♫
---------------------------------------------------
 音量:{:.2} 状态:{} 音频数量:{}
 音频/歌单:{}
---------------------------------------------------
 ♪   ♪   ♪   ♪   ♪   ♪   ♪   ♪   ♪   ♪   ♪   ♪   ♪
---------------------------------------------------
",
        sink.volume(),
        if sink.is_paused() {
            "暂停中"
        } else if !sink.is_paused() {
            "播放中"
        } else {
            "未知状态"
        },
        files_count,
        audio_name
    );
}

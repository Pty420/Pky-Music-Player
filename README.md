# 支持 x86 Windows、x86 Linux、loongarch64 Linux

# PkyMusic
这是一个基于 Rust 中的 Rodio 开发的简易CLI命令行音频播放器

## 介绍
这个项目是作为 Rust 初学者的我开发的第一个小项目，虽然功能受限，但勉强可以使用

## 功能
1. 播放本地音频文件或识别指定文件夹内所有音频文件并顺序播放

2. 支持暂停、调节音量

3. 播放单个文件时可循环播放和单次播放

## 使用说明
1. 首先确保你已经拥有 Rust 环境,并获取了本项目

2. 运行方法

如果你获取的是源码，那么在项目根目录下打开终端，输入以下命令
```bash  
cargo run 
```
如果你获取的是本项目的发行版，那么在可执行文件所在的目录下打开终端，输入以下命令
```bash
./文件名
```
若没能运行成功，可能是文件没有运行权限,请输入以下命令后再尝试运行
```bash
chmod +x 文件名
```
![运行效果](images/1.png "运行效果")

3. 接下来输入以下命令
```bash
play [参数] [音频文件路径]
```
或者
```bash
open [文件夹路径]
```
![播放效果](images/2.png "播放效果")

4. 如果不出意外，音乐应该是播放成功了！

在音乐播放过程中，程序仍会获取你的输入，这是程序在获取你的命令

```bash
#暂停
pause

#继续播放
play

#下一首
skip

#退出
quit

#设置音量
set volume [0 ~ 1]
```

### 注意:
1. play指令中的参数目前仅支持 -l 表示循环播放，如果不输入参数，就会默认单次播放

2. 你的路径如果是 / 开头，就表示绝对路径 

   (例如 /home/user/音乐/user/music.mp3)

   如果是非 / 开头，那么就表示相对默认音乐文件夹的路径 

   (例如你输入了 user/music.mp3，那么程序会自动补全为 /home/user/音乐/user/music.mp3)

部分路径可能会出现空格，尤其是在你的音频文件名之中，这你无需担心，因为程序会将 play [参数] 或 open 之后的所有内容都算作路径,例如 play user/m usic.mp3是有效的

注意：即使你是 Windows 用户，也请使用 / ，而不是 \

### x86架构版本可能会遇到的问题(以 Fedora Linux 43 为例)
1. 依赖缺失

请输入以下命令
```bash
sudo dnf install alsa-lib-devel
```

2. 其他bug

我会努力完善和修复:)

### loongarch(龙芯)架构版本可能会遇到的问题(以 Loongnix 25 为例)
1. 依赖缺失

请输入以下命令
```bash
sudo apt update && sudo apt install libasound2-dev gcc
```

2. 运行成功但播放音乐时报错

请尝试输入以下命令
```bash
sudo apt install pipewire pipewire-alsa pipewire-pulse

#安装完毕后重启音频服务
systemctl --user restart pipewire pipewire-pulse
```

3. 编译时的一些问题

若编译报错，请尝试删除项目根目录下的 Cargo.lock ,并修改配置文件 Cargo.toml ,如下面所示
```text
[package]
name = "pky_music"
version = "0.1.3"
edition = "2021"

[dependencies]
rodio = "0.17"
dirs = "5.0"
clearscreen = "2.0"
home = "=0.5.5"
quote = "=1.0.33"
unicode-ident = "=1.0.9"
```

4. 其他bug

我会努力完善和修复:)

use std::process::Command;
use std::thread;
use std::time::Duration;
use std::{
    env,
    fs::{self},
    path::PathBuf,
};

/// 获取 IconCache.db 路径
fn get_path() -> PathBuf {
    let local_path = env::var("LOCALAPPDATA").expect("获取 LOCALAPPDATA 失败");
    let mut path_buf = PathBuf::new();
    path_buf.push(local_path);
    path_buf.push("IconCache.db");
    path_buf
}

/// 删除文件
fn remove_file(path: &PathBuf) -> bool {
    let mut removed = false;

    if path.exists() {
        fs::remove_file(path).expect("删除失败");
        removed = true
    }

    removed
}

/// 重启文件资源管理器进程
fn restart_explorer() {
    // 结束文件资源管理器进程
    Command::new("taskkill")
        .args(&["/f", "/im", "explorer.exe"])
        .output()
        .expect("结束文件资源管理器进程失败");

    // 重启文件资源管理器
    let explorer_path = PathBuf::from(r"C:\Windows\explorer.exe");
    Command::new(explorer_path)
        .spawn()
        .expect("启动文件资源管理器失败");
}

#[allow(unused_assignments)]
fn main() {
    let mut tip_str = String::new();
    let path = get_path();
    let removed = remove_file(&path);
    if removed {
        restart_explorer();
        tip_str = String::from("清除成功...")
    } else {
        tip_str = String::from("未找到可清除的空图标...")
    }

    println!("{}", tip_str);
    thread::sleep(Duration::from_secs(2));
}

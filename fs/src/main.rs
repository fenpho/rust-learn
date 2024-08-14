use std::fs::File;
use std::io::{self, Read};

fn read_file(file_path: &str) -> io::Result<String> {
    // 尝试打开文件
    let mut file = File::open(file_path)?;

    // 创建一个字符串来存储文件内容
    let mut content = String::new();

    // 读取文件内容到字符串
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn main() {
    let file_path = r"D:\workspace\project\PatternLib\client\src-tauri\src\assets\raw\text.txt"; // 你的文件路径

    match read_file(file_path) {
        Ok(content) => {
            println!("File content:\n{}", content);
        }
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
        }
    }
}

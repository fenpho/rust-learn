// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x * 2;
//     println!("The value of x is: {}", x);
// }

use hex;
use reqwest;
use reqwest::header;
use serde_json::json;
use serde_json::Value;
use std::env;
use std::ffi::c_char;
use std::ffi::CString;
use std::ptr;
use uuid::Uuid;
extern crate libloading;
use base64::decode;

// 文件解密
async fn decrypt_file(url: &str) -> Result<(), reqwest::Error> {
    let response = reqwest::get(url).await?;

    println!("GET Response: {:?}", response.text().await?);

    Ok(())
}

// 文件加密
async fn encrypt_file(
    url: &str,
    ticket: &str,
    file_name: &str,
    file_list: Vec<&str>,
) -> Result<(), reqwest::Error> {
    // 生成一个新的 UUID
    // let uuid1 = Uuid::new_v4();
    // let uuid1 = "053371e6-2b05-4068-8b4e-a23c6672c4a0";
    // let uuid1 = Uuid::parse_str(uuid1).expect("Failed to parse UUID");

    // 将 Uuid 转换为 16 字节的字符串
    let uuid1 = Uuid::new_v4();
    let uuid1 = uuid1.as_bytes();
    let uuid1 = hex::encode(uuid1);

    // let uuid2 = Uuid::new_v4();
    let uuid2 = "c9988c66-231b-46d5-9619-354484619824";
    let uuid2 = Uuid::parse_str(uuid2).expect("Failed to parse UUID");

    // 将 Uuid 转换为 16 字节的字符串
    let uuid2 = uuid2.as_bytes();
    let uuid2 = hex::encode(uuid2);
    // println!("uuid 2: {}", uuid2);
    // let bytes = uuid2.as_bytes();

    // 将 UUID 打印出来
    // println!("Generated UUID: {}", uuid);

    // 构建自定义请求头
    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, ticket.parse().unwrap());
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .headers(headers)
        .json(&json!({
            "key": &uuid1,
        }))
        .send()
        .await?;

    let data = response.text().await?;
    println!("POST Response: {:?}", data);
    let parsed_json: Value = serde_json::from_str(&data).expect("Failed to parse JSON");
    let mut uuid3: Vec<u8> = vec![];
    if let Some(value) = parsed_json.get("data") {
        // 将值转换为字符串
        if let Some(encoded_str) = value.as_str() {
            // 解码 Base64 数据
            match decode(encoded_str) {
                Ok(decoded_bytes) => {
                    // 将字节数组转换为字符串，如果包含文本数据
                    uuid3 = decoded_bytes;
                    println!("解码后的数据: {}, {:?}", uuid3.len(), uuid3);
                }
                Err(e) => eprintln!("Base64 解码时出错：{}", e),
            }
        } else {
            eprintln!("键 'key' 的值不是字符串");
        }
    } else {
        eprintln!("找不到键 'key'");
    }

    if let Ok(current_dir) = env::current_dir() {
        // 构建 DLL 的相对路径
        println!("DLL loaded successfully.{:?}", current_dir);

        let dll_path = current_dir
            .join("src")
            .join("assets")
            .join("dll")
            .join("RplFile64.dll");

        // 使用 libloading crate 动态加载 DLL
        if let Ok(library) = libloading::Library::new(dll_path) {
            println!("DLL loaded successfully.");

            // 获取 DLL 中的函数
            unsafe {
                if let Ok(save_file) = library.get::<unsafe extern "C" fn(
                    *const c_char,
                    *const *const c_char,
                    usize,
                    *const c_char,
                    *const c_char,
                    *const c_char,
                    usize,
                ) -> i32>(b"SaveFile")
                {
                    // 调用 DLL 中的函数
                    let c_file_name = CString::new(file_name).unwrap();
                    let c_string: Vec<*const c_char> = file_list
                        .iter()
                        .map(|&s| {
                            (CString::new(s.to_string()).unwrap()).into_raw() as *const c_char
                        })
                        .collect();
                    let c_uuid1 = CString::new(uuid1).unwrap();
                    let c_uuid2 = CString::new(uuid2).unwrap();
                    println!(
                        "Params of save_file: {:?},\n {:?},\n {:?},\n {:?},\n {:?},\n {:?},\n {:?}",
                        c_file_name.as_ptr(),
                        c_string.as_ptr(),
                        c_string.len(),
                        c_uuid1.as_ptr(),
                        c_uuid2.as_ptr(),
                        uuid3.as_ptr(),
                        uuid3.len(),
                    );

                    let result = save_file(
                        c_file_name.as_ptr() as *const c_char,
                        c_string.as_ptr() as *const *const c_char,
                        c_string.len(),
                        c_uuid1.as_ptr() as *const c_char,
                        c_uuid2.as_ptr() as *const c_char,
                        uuid3.as_ptr() as *const c_char,
                        uuid3.len(),
                    );
                    println!("\n");
                    println!("Result of save_file: {}", result);
                } else {
                    println!("Failed to get save_file function from DLL.");
                }
            }
        } else {
            println!("Failed to load DLL.");
        }
    } else {
        println!("Failed to get current working directory.");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Ok(exe_dir) = env::current_exe() {
        print!("exe dir: {:?}", exe_dir);
    
        } else {
            println!("Failed to get current working directory.");
        }
    if let Err(err) = encrypt_file(
        "https://test2.fangzhiyun.cn:8000/patternlib/cipher/encrypt",
        "72418ba25d07415cbb8e53cc55fdfef3",
        "d:\\Documents\\测试\\output\\撒大大.fzy",
        vec!["d:\\Documents\\测试\\raw\\撒大大.txt"],
    )
    .await
    {
        eprintln!("Error encript file: {:?}", err);
    }

    // 发送 POST 请求
    // if let Err(err) = decrypt_file().await {
    //     eprintln!("Error decrypt file: {:?}", err);
    // }
}

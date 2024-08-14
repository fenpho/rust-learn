extern crate crypto;
extern crate rand;

use crypto::aes::KeySize;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};
use crypto::symmetriccipher::{Decryptor, Encryptor};
use rand::rngs::OsRng;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use uuid::Uuid;

const AES_KEY: &str = "c9988c66-231b-46d5-9619-354484619824";

fn aes_encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    // 创建 AES 加密器
    println!("key len: {:?}", key.len());
    let mut encryptor =
        crypto::aes::cbc_encryptor(KeySize::KeySize256, key, &[0u8; 16], PkcsPadding);

    // 加密数据
    let mut ciphertext = Vec::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(data);
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut ciphertext);

    // encryptor
    //     .encrypt(&mut read_buffer, &mut write_buffer, true)
    //     .unwrap();
    match encryptor.encrypt(&mut read_buffer, &mut write_buffer, true) {
        Ok(BufferResult::BufferUnderflow) => {
            // 加密成功，打印加密前后的长度
            println!("Before encryption: {}", data.len());
            println!("After encryption: {}", ciphertext.len());
        }
        Err(err) => {
            // 加密失败，打印错误信息
            eprintln!("Encryption error: {:?}", err);
        }
        _ => {}
    }

    // 打印加密前后的长度
    println!("Before encryption: {}", data.len());
    println!("After encryption: {}", ciphertext.len());

    ciphertext
}

fn aes_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    // 创建 AES 解密器
    let mut decryptor =
        crypto::aes::cbc_decryptor(KeySize::KeySize256, key, &[0u8; 16], PkcsPadding);

    // 解密数据
    let mut plaintext = Vec::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(data);
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut plaintext);

    decryptor
        .decrypt(&mut read_buffer, &mut write_buffer, true)
        .unwrap();

    // 打印解密前后的长度
    println!("Before decryption: {}", data.len());
    println!("After decryption: {}", plaintext.len());
    plaintext
}

fn encrypt_files(files: &[&str], output_file: &str, key: &[u8]) -> io::Result<()> {
    let mut output = File::create(output_file)?;

    for file in files {
        let mut input_data = Vec::new();
        File::open(file)?.read_to_end(&mut input_data)?;

        let encrypted_data = aes_encrypt(&input_data, key);
        // 打印加密前后的长度
        println!("Before encryption: {}", input_data.len());
        println!("After encryption: {}", encrypted_data.len());

        output.write_all(&encrypted_data)?;
        println!("File {} encrypted.", file);
        // 打印已写入文件的长度
        println!("Written to file: {}", encrypted_data.len());
        println!("File {} encrypted.", file);
    }

    Ok(())
}

fn decrypt_file(input_file: &str, output_directory: &str, key: &[u8]) -> io::Result<()> {
    let mut input_data = Vec::new();
    File::open(input_file)?.read_to_end(&mut input_data)?;

    let decrypted_data = aes_decrypt(&input_data, key);
    let mut current_position = 0;
    let mut file_list = Vec::new();

    // 从解密的数据中提取文件列表
    while current_position < decrypted_data.len() {
        let file_name_length = decrypted_data[current_position] as usize;
        current_position += 1;

        let file_name = String::from_utf8_lossy(
            &decrypted_data[current_position..current_position + file_name_length],
        );
        current_position += file_name_length;

        file_list.push(file_name.into_owned());
    }

    print!("file list {:?}", file_list);
    // 根据文件列表，逐个还原文件
    for file_name in file_list {
        let file_data_length = decrypted_data[current_position] as usize;
        current_position += 1;

        let file_data = &decrypted_data[current_position..current_position + file_data_length];
        current_position += file_data_length;

        let output_file_path = format!("{}{}", output_directory, file_name);
        File::create(output_file_path)?.write_all(file_data)?;
        println!("File {} decrypted.", file_name);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // 将密钥字符串转换为字节数组
    let key: Vec<u8> = AES_KEY.bytes().collect();

    // 将密钥字符串转换为字节数组
    let uuid1 = "053371e6-2b05-4068-8b4e-a23c6672c4a0";
    let uuid1 = Uuid::parse_str(uuid1).expect("Failed to parse UUID");

    // 将 Uuid 转换为 16 字节的字符串
    let key = uuid1.as_bytes();

    // 要加密的文件列表
    let files_to_encrypt = vec![
        r"d:\Documents\测试\raw\aaa-那.docx",
        r"d:\Documents\测试\raw\text.TXT",
    ];

    // 加密文件到一个 .fzy 文件
    encrypt_files(&files_to_encrypt, "encrypted.fzy", key)?;

    // 解密文件到指定目录
    decrypt_file("encrypted.fzy", "output_directory/", key)?;

    Ok(())
}

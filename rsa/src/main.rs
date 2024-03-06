use rand::rngs::OsRng;
use rsa::{PaddingScheme, PublicKey, RSAPrivateKey, RSAPublicKey};
use uuid::Uuid;

fn main() {
    // 生成随机数生成器
    let mut rng = OsRng;

    // 构建 RSA 密钥对
    let bits = 2048;
    let private_key = RSAPrivateKey::new(&mut rng, bits).expect("Failed to generate private key");
    let public_key = RSAPublicKey::from(&private_key);

    // 打印公钥和私钥
    println!("Public Key: {:?}", public_key);
    println!("Private Key: {:?}", private_key);

    // 待加密的字符串
    let plaintext = "c9988c66-231b-46d5-9619-354484619824";
    // 获取 UUID 的字节数组
    let uuid = Uuid::parse_str(plaintext).expect("Failed to parse UUID");
    let bytes = uuid.as_bytes();

    // 打印 16 字节的十六进制字符串
    let hex_string = bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>();
    println!("16 Byte Hex String: {}", hex_string);

    // 将字符串转换为字节序列
    let data_to_encrypt = uuid.as_bytes();

    // 使用公钥进行加密
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let encrypted_data = public_key
        .encrypt(&mut rng, padding, data_to_encrypt)
        .expect("Failed to encrypt data");

    // 打印加密后的数据
    println!("Encrypted Data: {:?}", encrypted_data);

    // 使用私钥进行解密
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let decrypted_data = private_key
        .decrypt(padding, &encrypted_data)
        .expect("Failed to decrypt data");

    // 打印解密后的数据
    let decrypted_string =
        Uuid::from_slice(&decrypted_data).expect("Failed to restore UUID from bytes");
    let hex_string = decrypted_data
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>();
    println!("after 16 Byte Hex String: {}", hex_string);
    println!("Decrypted Data: {}", decrypted_string);
}

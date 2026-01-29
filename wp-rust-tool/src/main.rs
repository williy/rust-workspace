use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;

#[derive(Debug, Default)]
struct WpDbConfig {
    db_name: String,
    db_user: String,
    db_password: String,
}

fn main() {
    let path = "wp-config.php";

    // --- 1. 抽出 (Extraction) ---
    let content = fs::read_to_string(path).expect("wp-config.php が読み込めません");
    let re = Regex::new(r#"(?i)define\s*\(\s*['"](.+?)['"]\s*,\s*['"](.+?)['"]\s*\);"#).unwrap();
    let mut config = WpDbConfig::default();

    for cap in re.captures_iter(&content) {
        match &cap[1] {
            "DB_NAME" => config.db_name = cap[2].to_string(),
            "DB_USER" => config.db_user = cap[2].to_string(),
            "DB_PASSWORD" => config.db_password = cap[2].to_string(),
            _ => {}
        }
    }

    println!("--- WordPress Configuration Extracted ---");
    println!("Database: {}", config.db_name);
    println!("User    : {}", config.db_user);

    // --- 2. 暗号化 (Encryption) ---
    // ※ B-Sheet B-301: 本来は環境変数に逃がすべき「聖域」の鍵
    let key_data = b"an example very very secret key.";
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(key_data);
    let cipher = Aes256Gcm::new(key);

    if !config.db_password.is_empty() {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, config.db_password.as_bytes())
            .expect("Encryption failed");

        let mut file = File::create("db_pass.enc").expect("Failed to create enc file");
        file.write_all(&nonce).unwrap();
        file.write_all(&ciphertext).unwrap();
        println!("Password: [ENCRYPTED AND SAVED TO db_pass.enc]");
    }

    // --- 3. 復号化 (Decryption / Verification) ---
    println!("\n--- Decryption Test (Phase 3) ---");
    let enc_data = fs::read("db_pass.enc").expect("Failed to read enc file");

    // B-202 の設計通り、先頭 12 バイト（ナンス）と本体（暗号文）を切り分ける
    let (nonce_part, encrypted_part) = enc_data.split_at(12);
    let nonce = aes_gcm::Nonce::from_slice(nonce_part);

    match cipher.decrypt(nonce, encrypted_part) {
        Ok(decrypted_bytes) => {
            let password_str = String::from_utf8(decrypted_bytes).expect("Invalid UTF-8");
            println!("🔓 Decrypted Password: {}", password_str);
            println!("✅ 資産の可逆性を確認しました。道理は通りました。");
        }
        Err(_) => {
            println!("❌ 復号失敗。データが改ざんされているか、鍵が一致しません。");
        }
    }
}

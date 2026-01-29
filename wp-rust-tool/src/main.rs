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

    // 1. ファイル読み込み
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            println!("Error: {} が見つかりません。", path);
            return;
        }
    };

    // 2. 正規表現で抽出
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

    // 3. 抽出結果の処理
    if config.db_name.is_empty() {
        println!("設定が見つかりませんでした。");
        return;
    }

    println!("--- WordPress Configuration Extracted ---");
    println!("Database: {}", config.db_name);
    println!("User    : {}", config.db_user);

    // 4. パスワードの暗号化保存（B-Sheet B-201/202 に基づく実装）
    if !config.db_password.is_empty() {
        // 32バイトの固定鍵 (B-301: 将来的に環境変数化予定)
        let key_data = b"an example very very secret key.";
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(key_data);
        let cipher = Aes256Gcm::new(key);

        // ユニークなナンス（12バイト）の生成
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        // 暗号化
        let ciphertext = cipher
            .encrypt(&nonce, config.db_password.as_bytes())
            .expect("Encryption failed");

        // ファイル保存 (Nonce + Ciphertext)
        let mut file = File::create("db_pass.enc").expect("Failed to create enc file");
        file.write_all(&nonce).unwrap();
        file.write_all(&ciphertext).unwrap();

        println!("Password: [ENCRYPTED AND SAVED TO db_pass.enc]");
    }
}

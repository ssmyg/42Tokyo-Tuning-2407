use rand::distributions::Alphanumeric;
use base64::{encode};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::Rng;

use crate::errors::AppError;

use opentelemetry_auto_span::auto_span;

// pub fn generate_session_token() -> String {
//     let mut rng = rand::thread_rng();
//     let token: String = (0..30)
//         .map(|_| {
//             let idx = rng.gen_range(0..62);
//             let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
//             chars[idx] as char
//         })
//         .collect();
//     token
// }

/// セッショントークンを生成する関数
#[auto_span]
pub fn generate_session_token() -> String {
    // 32バイトのランダムなデータを生成
    let random_bytes: Vec<u8> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .collect();

    // ベース64エンコーディングを使用して文字列に変換
    encode(random_bytes)
}

#[auto_span]
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let password_bytes = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    match argon2.hash_password(password_bytes, &salt) {
        Ok(hashed_password_bytes) => Ok(hashed_password_bytes.to_string()),
        Err(_) => Err(AppError::InternalServerError),
    }
}

#[auto_span]
pub fn verify_password(hashed_password: &str, input_password: &str) -> Result<bool, AppError> {
    let input_password_bytes = input_password.as_bytes();
    let parsed_hash = match PasswordHash::new(hashed_password) {
        Ok(hash) => hash,
        Err(_) => return Err(AppError::InternalServerError),
    };
    match Argon2::default().verify_password(input_password_bytes, &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

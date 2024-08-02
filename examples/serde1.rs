use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, KeyInit};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

//32 bytes
const KEY: &[u8] = b"01234567890123456789012345678901";
//12 bytes
const NONCE: &[u8] = b"012345678901";

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    name: String,
    #[serde(rename = "myAge")]
    age: u8,
    date_of_birth: DateTime<Utc>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    skils: Vec<String>,
    state: WorkState,
    #[serde(serialize_with = "b64_encode", deserialize_with = "b64_decode")]
    data: Vec<u8>,
    #[serde(
        serialize_with = "serialize_encrypt",
        deserialize_with = "deserialize_decrypt"
    )]
    sensitive: String,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    url: Vec<http::Uri>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "details")]
enum WorkState {
    Working(String),
    OnLeave(DateTime<Utc>),
    Terminated,
}

fn b64_encode<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encoded = URL_SAFE_NO_PAD.encode(data);
    serializer.serialize_str(&encoded)
}

fn b64_decode<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encoded = String::deserialize(deserializer)?;
    let decoded = URL_SAFE_NO_PAD
        .decode(encoded)
        .map_err(serde::de::Error::custom)?;
    Ok(decoded)
}

fn serialize_encrypt<S>(data: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let encrypted = encrypt(data.as_bytes()).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&encrypted)
}

fn deserialize_decrypt<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encrypted = String::deserialize(deserializer)?;
    let decrypted = decrypt(&encrypted).map_err(serde::de::Error::custom)?;
    let decrypted = String::from_utf8(decrypted).map_err(serde::de::Error::custom)?;
    Ok(decrypted)
}

fn encrypt(data: &[u8]) -> Result<String> {
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let ciphertext = cipher.encrypt(NONCE.into(), data).unwrap();
    Ok(URL_SAFE_NO_PAD.encode(ciphertext))
}

fn decrypt(encoded: &str) -> Result<Vec<u8>> {
    let decoded: Vec<u8> = URL_SAFE_NO_PAD.decode(encoded)?;
    let cipher = ChaCha20Poly1305::new(KEY.into());
    let decrypted = cipher.decrypt(NONCE.into(), &decoded[..]).unwrap();
    Ok(decrypted)
}

fn main() -> Result<()> {
    // let state = WorkState::Working("Rust Egineer".to_string());
    let state1 = WorkState::OnLeave(Utc::now());
    let user = User {
        name: "maya".to_string(),
        age: 18,
        date_of_birth: Utc::now(),
        skils: vec!["rust".to_string(), "python".to_string()],
        state: state1,
        data: vec![1, 2, 3, 4, 5],
        sensitive: "secret".to_string(),
        url: vec!["http://www.example.com".parse()?],
    };

    let json = serde_json::to_string(&user)?;
    println!("{}", json);

    let user: User = serde_json::from_str(&json)?;
    println!("{:?}", user);
    println!("{:?}", user.url[0].host());

    Ok(())
}

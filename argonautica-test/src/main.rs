extern crate argonautica;

use argonautica::{Hasher, Verifier};

fn get_scret_key()-> String {
    "secret+key".repeat(8)
}

fn get_salt()-> String {
    "salt".repeat(8)
}

fn hashing(pwd: String, salt: String) -> String {
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(&pwd)
        .with_secret_key(get_scret_key())
        .with_salt(&salt)
        .hash()
        .unwrap();

    println!("{}", &hash);

    return hash;
}

fn hashing_no_salt(pwd: String) -> String {
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(&pwd)
        .with_secret_key(get_scret_key())
        .hash()
        .unwrap();

    println!("{}", &hash);

    return hash;
}

fn verifying(hash: String, pwd: String) {
    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(&hash)
        .with_password(&pwd)
        .with_secret_key(get_scret_key())
        .verify()
        .unwrap();

    println!("{:?}", is_valid);
    assert!(is_valid);
}

fn main() {
    let hash = hashing("password".to_string(), get_salt());

    let hash_no_salt = hashing_no_salt("password".to_string());
    let hash_no_salt_2 = hashing_no_salt("password".to_string());


   // 如果不使用salt 每次生成的HASH是不同的.
    verifying(hash, "password".to_string());
    verifying(hash_no_salt, "password".to_string());
    verifying(hash_no_salt_2, "password".to_string());
}
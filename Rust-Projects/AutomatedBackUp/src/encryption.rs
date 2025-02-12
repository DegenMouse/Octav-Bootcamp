use std::fs::File;
use std::io::{Read, Write};
use crate::error::BackupResult;

use orion::hazardous::{
    aead::xchacha20poly1305::{seal, open, Nonce, SecretKey},
    mac::poly1305::POLY1305_OUTSIZE,
    stream::xchacha20::XCHACHA_NONCESIZE,
};

use orion::hazardous::stream::chacha20::CHACHA_KEYSIZE;
use orion::kdf::{derive_key, Password, Salt};
use rand_core::{OsRng, RngCore};

fn get_random(dest: &mut [u8]) {
    RngCore::fill_bytes(&mut OsRng, dest);
}

fn nonce() -> Vec<u8> {
    let mut randoms: [u8; 24] = [0; 24];
    get_random(&mut randoms);
    randoms.to_vec()
}

fn auth_tag() -> Vec<u8> {
    let mut randoms: [u8; 32] = [0; 32];
    get_random(&mut randoms);
    randoms.to_vec()
}

fn simple_split_encrypted(cipher_text: &[u8]) -> (Vec<u8>, Vec<u8>) {
    (
        cipher_text[..CHACHA_KEYSIZE].to_vec(),
        cipher_text[CHACHA_KEYSIZE..].to_vec(),
    )
}

fn create_key(password: String, nonce: Vec<u8>) -> BackupResult<SecretKey> {
    let password = Password::from_slice(password.as_bytes())?;
    let salt = Salt::from_slice(nonce.as_slice())?;
    let kdf_key = derive_key(&password, &salt, 15, 1024, CHACHA_KEYSIZE as u32)?;
    let key = SecretKey::from_slice(kdf_key.unprotected_as_bytes())?;
    Ok(key)
}

fn encrypt_core(
    dist: &mut File,
    contents: Vec<u8>,
    key: &SecretKey,
    nonce: Nonce,
) -> BackupResult<()> {
    let ad = auth_tag();
    let output_len = match contents.len().checked_add(POLY1305_OUTSIZE + ad.len()) {
        Some(min_output_len) => min_output_len,
        None => anyhow::bail!("Plaintext is too long"),
    };

    let mut output = vec![0u8; output_len];
    output[..CHACHA_KEYSIZE].copy_from_slice(ad.as_ref());
    seal(key, &nonce, contents.as_slice(), Some(ad.clone().as_slice()), &mut output[CHACHA_KEYSIZE..])?;
    dist.write(output.as_slice())?;
    Ok(())
}

fn decrypt_core(
    dist: &mut File,
    contents: Vec<u8>,
    key: &SecretKey,
    nonce: Nonce
) -> BackupResult<()> {
    let split = simple_split_encrypted(contents.as_slice());
    let mut output = vec![0u8; split.1.len() - POLY1305_OUTSIZE];

    open(key, &nonce, split.1.as_slice(), Some(split.0.as_slice()), &mut output)?;
    dist.write(output.as_slice())?;
    Ok(())
}


const CHUNK_SIZE: usize = 128; // The size of the chunks you wish to split the stream into.

pub fn encrypt_large_file(
    file_path: &str,
    output_path: &str,
    password: String
) -> BackupResult<()> {
    let mut source_file = File::open(file_path)?;
    let mut dist = File::create(output_path)?;

    let mut src = Vec::new();
    source_file.read_to_end(&mut src)?;

    let nonce = nonce();

    dist.write(nonce.as_slice())?;
    let key = create_key(password, nonce.clone())?;
    let nonce = Nonce::from_slice(nonce.as_slice())?;

    for (_, src_chunk) in src.chunks(CHUNK_SIZE).enumerate() {
        encrypt_core(&mut dist, src_chunk.to_vec(), &key, nonce)?;
    }

    Ok(())
}

pub fn decrypt_large_file(
    file_path: &str, 
    output_path: &str,
    password: String
) -> BackupResult<()> {
    let mut input_file = File::open(file_path)?;
    let mut output_file = File::create(output_path)?;

    let mut src: Vec<u8> = Vec::new();
    input_file.read_to_end(&mut src)?;

    let nonce = src[..XCHACHA_NONCESIZE].to_vec();

    src = src[XCHACHA_NONCESIZE..].to_vec();

    let key = create_key(password, nonce.clone())?;
    let nonce = Nonce::from_slice(nonce.as_slice())?;

    for (count, src_chunk) in src.chunks(CHUNK_SIZE + CHACHA_KEYSIZE + POLY1305_OUTSIZE).enumerate() {
        println!("Decrypting chunk {}", count);
        decrypt_core(&mut output_file, src_chunk.to_vec(), &key, nonce)?;
    }

    Ok(())
}
use aes::Aes128;
use block_modes::{BlockMode, BlockModeError, Ecb};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use rocket::futures::AsyncReadExt;
use std::{borrow::Borrow, ops::Add, time::{Duration, SystemTime}};

use josekit::{
    jwe::{JweHeader, A128KW},
    jwt::{self, JwtPayload},
    JoseError, Map, Value,
};

const SECRET_KEY: &str = "XwKsGlMcdPMEhR1B";
pub struct Token<'a> {
    jwt_secret: &'a str,
    aes_key: String,
    aes_iv: String 
}
#[derive(Debug)]
pub enum TokenError {
    JoseError,
    TokenExp,
    AesError
}

type Aes128Ecb = Ecb<Aes128, Pkcs7>;
impl <'a> Token<'a> {
    pub fn new() -> Self {
        let k = SECRET_KEY;
        Token {
            jwt_secret: "0123456789ABCDEF",
            aes_iv: "0123456789abcdef".to_string(),
            aes_key: k.to_string()
       }
    }
    pub fn encode(&self, claim: Map<String, Value>, exp: u64) -> String {
        let mut header = JweHeader::new();
        header.set_token_type("JWT");
        header.set_content_encryption("A128CBC-HS256");
        let mut payload = JwtPayload::new();
        for i in claim {
            payload.set_claim(&i.0, Some(i.1));
        }
        let time = SystemTime::now();
        let time = time.add(Duration::from_secs(exp));
        payload.set_expires_at(&time);
        // Encrypting JWT
        let encrypter = A128KW.encrypter_from_bytes(&self.jwt_secret).unwrap();
        let jwt = jwt::encode_with_encrypter(&payload, &header, &encrypter).unwrap();
        jwt
    }
    pub fn verify(&self, token: &str) -> Result<Map<String, Value>, TokenError> {
        let rs = self.decode(token);
        match rs {
            Ok(payload) => {
                let exp = payload.expires_at().unwrap();
                if exp > SystemTime::now() {
                    Ok(payload.into())
                } else {
                    Err(TokenError::TokenExp)
                }
            }
            Err(_e) => Err(TokenError::JoseError),
        }
    }
    pub fn decode(&self, str: &str) -> Result<JwtPayload, JoseError> {
        let decrypter = A128KW.decrypter_from_bytes(&self.jwt_secret)?;
        let (payload, _header) = jwt::decode_with_decrypter(str, &decrypter)?;
        Ok(payload)
    }
    pub fn aes_encode(&self, plaintext: &[u8]) -> Result<String, BlockModeError> {
        let cipher = Aes128Ecb::new_var(self.aes_key.as_bytes(), self.aes_iv.as_bytes()).unwrap();
        // buffer must have enough space for message+padding
        // copy message to the buffer
        let pos = plaintext.len();
        let len = pos + 16 - pos % 16;
        let mut buffer = Vec::with_capacity(len);//[0u8; len];
        for _a in 0..len{
            buffer.push(0);
        }
        println!("len = {}, pos = {}", buffer.len(), pos);
        buffer[..pos].copy_from_slice(plaintext);
        let ciphertext = cipher.encrypt(&mut buffer, pos)?;
        Ok(base64::encode(ciphertext))
    }
    pub fn aes_decode(&self, str: &str) -> Result<Vec<u8>, TokenError> {
        let cipher = Aes128Ecb::new_var(self.aes_key.as_bytes(), self.aes_iv.as_bytes()).unwrap();
        let mut buf = base64::decode(str).unwrap();
                let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
        Ok(decrypted_ciphertext.to_vec())
    }
}

use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, BlockModeError, Ecb};
use hex_literal::hex;
use rocket::futures::AsyncReadExt;
use std::{
    borrow::Borrow,
    ops::Add,
    time::{Duration, SystemTime},
};

use josekit::{
    jwe::{JweHeader, A128KW},
    jwt::{self, JwtPayload},
    JoseError, Map, Value,
};

const SECRET_KEY: &str = "XwKsGlMcdPMEhR1B";
const IV: &str = SECRET_KEY;
pub struct Token<'a> {
    jwt_secret: &'a str,
    aes_key: String,
    aes_iv: String,
}
#[derive(Debug)]
pub enum TokenError {
    JoseError,
    TokenExp,
    AesError,
    PointFmtError,
}
pub type Res<T> = Result<T, TokenError>;
pub trait ComFrom<T>: Sized {
    fn com_from(_t: T) -> Res<Self>;
}
impl ComFrom<Vec<u8>> for Vec<u8> {
    fn com_from(s: Vec<u8>) -> Res<Self> {
        Ok(s)
    }
}
type Aes128Ecb = Ecb<Aes128, Pkcs7>;
impl<'a> Token<'a> {
    pub fn new() -> Self {
        Token {
            jwt_secret: "0123456789ABCDEF",
            aes_iv: IV.to_string(),
            aes_key: SECRET_KEY.to_string(),
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
        let mut buffer = Vec::with_capacity(len); //[0u8; len];
        for _a in 0..len {
            buffer.push(0);
        }
        buffer[..pos].copy_from_slice(plaintext);
        let ciphertext = cipher.encrypt(&mut buffer, pos)?;
        Ok(base64::encode(ciphertext))
    }
    pub fn aes_decode<T>(&self, str: &str) -> Res<T>
    where
        T: ComFrom<Vec<u8>>,
    {
        let cipher = Aes128Ecb::new_var(self.aes_key.as_bytes(), self.aes_iv.as_bytes()).unwrap();
        let mut buf = base64::decode(str).unwrap();
        let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
         T::com_from(decrypted_ciphertext.to_vec())
    }
}

use std::{ops::Add, time::{Duration, SystemTime}};

use josekit::{JoseError, Map, Value, jwe::{JweHeader, A128KW}, jwt::{self, JwtPayload}};
pub struct Token{
    secret: String
}
pub enum TokenError{
    JoseError,
    TokenExp
}

impl Token{
    pub fn new() -> Self{
        Token{
            secret: "0123456789ABCDEF".to_string()
        }
    }
    pub fn encode(&self, claim: Map<String, Value>, exp: u64) -> String{
        let mut header = JweHeader::new();
        header.set_token_type("JWT");
        header.set_content_encryption("A128CBC-HS256");
        let mut payload = JwtPayload::new();
        for i in claim{
            payload.set_claim(&i.0, Some(i.1));
        }
        let time = SystemTime::now();
        let time = time.add(Duration::from_secs(exp));
        payload.set_expires_at(&time);
        // Encrypting JWT
        let encrypter = A128KW.encrypter_from_bytes(&self.secret).unwrap();
        let jwt = jwt::encode_with_encrypter(&payload, &header, &encrypter).unwrap();
        jwt
    }
    pub fn verify(&self, token: &str) -> Result<Map<String, Value>, TokenError>{
        let rs = self.decode(token);
        match rs {
            Ok(payload) => {
                let exp = payload.expires_at().unwrap();
                if exp > SystemTime::now() {
                    Ok(payload.into())
                }else{
                    Err(TokenError::TokenExp)
                }
            }
            Err(_e) => Err(TokenError::JoseError)
        } 
    }
    pub fn decode(&self, str: &str) -> Result<JwtPayload, JoseError>{
        let decrypter = A128KW.decrypter_from_bytes(&self.secret)?;
        let (payload, _header) = jwt::decode_with_decrypter(str, &decrypter)?;
         Ok(payload) 
    }
}
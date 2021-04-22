use crate::token::ComFrom;

use super::token::Token;
use super::Behavior;
use super::{
    captcha::{get_captcha, DoubleBuffer},
    token::Res,
    token::TokenError,
};
use josekit::{
    jwe::{JweHeader, A128KW},
    jwt::{self, JwtPayload},
    JoseError, Map, Value,
};
use rocket::http::Header;
use rocket::routes;
use rocket::Response;
use rocket::{http::hyper::header::CONTENT_TYPE, post};
use rocket::{tokio::runtime::Runtime, Shutdown, State};
use rocket_contrib::{json::Json, serve::StaticFiles};
use serde::{Deserialize, Serialize};
use serde_json::{json, Result};
use std::io::Cursor;
use std::{sync::Arc, thread};
extern crate base64;
#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    captchaType: String,
    pointJson: String,
    token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}
impl ComFrom<Vec<u8>> for Point {
    fn com_from(s: Vec<u8>) -> Res<Point> {
        let str = String::from_utf8(s).unwrap();
        if let Ok(p) = serde_json::from_str(str.as_str()) {
            Ok(p)
        } else {
            Err(TokenError::PointFmtError)
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CaptchaType {
    captchaType: String,
}
pub struct Phx {
    thread: Option<thread::JoinHandle<()>>,
    handle: Shutdown,
}
#[post("/", format = "json", data = "<cap_type>")]
fn captcha_get(state: State<DoubleBuffer>, cap_type: Json<CaptchaType>) -> Response {
    println!("cap_type = {:?}", cap_type.0.captchaType);
    let mut response = Response::new();
    let header = Header::new(CONTENT_TYPE.as_str(), "application/json");
    response.adjoin_header(header);
    match get_captcha(Arc::clone(&state)) {
        Ok(p) => {
            use std::io::Cursor;
            let (mut dst_block_buff, mut dst_image_buff) =
                (Cursor::new(vec![]), Cursor::new(vec![]));
            p.dst_block
                .write_to(&mut dst_block_buff, image::ImageOutputFormat::Png)
                .unwrap();
            let dst_block_base64 = base64::encode(dst_block_buff.get_ref());
            p.dst_image
                .write_to(&mut dst_image_buff, image::ImageOutputFormat::Png)
                .unwrap();
            let dst_image_base64 = base64::encode(dst_image_buff.get_ref());
            let token = Token::new();
            let mut claim = Map::new();
            claim.insert("x".to_string(), json!(p.x));
            claim.insert("y".to_string(), json!(p.y));
            let token = token.encode(claim, 30);
            let body = json!({
                    "repCode": "0000",
                    "repData": {
                        "originalImageBase64": dst_image_base64,
                         "jigsawImageBase64": dst_block_base64,
                         "point":{
                            "x": p.x,
                            "y": p.y
                         },
                        "token": token, //一次校验唯一标识
                        "result": false,
                        "opAdmin": false
                    },
                    "success": true,
                    "error": false
            });
            response.set_sized_body(body.to_string().len(), Cursor::new(body.to_string()));
            response
        }

        Err(_) => {
            let body = json!({
                    "repCode": "0001",
                    "success": false,
                    "error": true
            });
            response.set_sized_body(body.to_string().len(), Cursor::new(body.to_string()));
            response
        }
    }
}
#[post("/", format = "json", data = "<position>")]
fn captcha_check(_state: State<DoubleBuffer>, position: Json<Position>) -> Response {
    let pos = position.0;
    // check token
    let token = Token::new();
    let body = match token.verify(&pos.token) {
        Ok(claim) => {
            //
            match token.aes_decode::<Point>(pos.pointJson.as_str()) {
                Ok(point) => {
                    let diff = claim["x"].as_i64().unwrap() - point.x;
                   let r = if diff < 5 || diff > -5 {
                        json!({
                        "repCode": "0000",
                        "repData": {
                            "captchaType": "blockPuzzle",
                            "token": pos.token,
                            "result": true,
                            "opAdmin": false
                        },
                        "success": true,
                        "error": false
                            })
                    } else {
                        json!({
                        "repCode": "0001",
                        "success": false,
                        "error": true
                            })
                    };
                    r
                },
                Err(_) => {
                    json!({
                    "repCode": "0001",
                    "success": false,
                    "error": true
                        })
                }
            }
        }
        Err(e) => {
            println!("token error = {:?}", e);
            json!({
            "repCode": "0001",
            "success": false,
            "error": true
                })
        }
    };
    let mut response = Response::new();
    let header = Header::new(CONTENT_TYPE.as_str(), "application/json");
    response.adjoin_header(header);
    response.set_sized_body(body.to_string().len(), Cursor::new(body.to_string()));
    response
}
impl Phx {
    pub fn new(dst_double_buffer: DoubleBuffer) -> Self {
        let rocket = rocket::ignite()
            .mount("/captcha/get", routes![captcha_get])
            .mount("/captcha/check", routes![captcha_check])
            .mount("/static/", StaticFiles::from("./priv/static"))
            .mount("/captcha-web/", StaticFiles::from("./priv/"))
            .manage(dst_double_buffer);
        let handle = rocket.shutdown();
        let thread = thread::spawn(move || {
            let r = rocket.launch();
            let rt = Runtime::new().unwrap();
            let _ = rt.block_on(r);
        });
        Phx {
            thread: Some(thread),
            handle: handle,
        }
    }
}
impl Behavior for Phx {
    fn init(&mut self) -> bool {
        true
    }
    fn terminate(&mut self) -> bool {
        let handle = self.handle.clone();
        handle.shutdown();
        if let Some(thread) = self.thread.take() {
            if let Ok(_) = thread.join() {
                println!("phx terminate ");
            }
        }
        true
    }
    fn run(&mut self) -> bool {
        true
    }
}

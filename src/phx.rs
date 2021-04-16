use crate::captcha::{get_captcha, DoubleBuffer};
use crate::io::Cursor;
use crate::Behavior;
use rocket::http::hyper::header::CONTENT_TYPE;
use rocket::http::Header;
use rocket::Response;
use rocket::{tokio::runtime::Runtime, Shutdown, State};
use rocket_contrib::{json::Json, serve::StaticFiles};
use std::{sync::Arc, thread};
use serde_json::{Result, json};
use josekit::{JoseError, Map, Value, jwe::{JweHeader, A128KW}, jwt::{self, JwtPayload}};
use crate::token::Token;
use serde::{Deserialize, Serialize};
extern crate base64;
#[derive(Deserialize, Serialize)]
pub struct Position{
     captchaType: String,
	 pointJson: String, 
	 token: String
}
pub struct Phx {
    thread: Option<thread::JoinHandle<()>>,
    handle: Shutdown,
}
#[get("/")]
fn captcha_get(state: State<DoubleBuffer>) -> Response {
    let mut response = Response::new();
    let header = Header::new(CONTENT_TYPE.as_str(), "application/json");
    response.adjoin_header(header);
    match get_captcha(Arc::clone(&state)) {
        Ok(p) => {
            use std::io::Cursor;
            let (mut dst_block_buff, mut dst_image_buff) = (Cursor::new(vec![]), Cursor::new(vec![]));
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
#[post("/",format = "json", data = "<position>")]
fn captcha_check(state: State<DoubleBuffer>, position: Json<Position>) -> Response {
    let mut response = Response::new();
    let header = Header::new(CONTENT_TYPE.as_str(), "application/json");
    response.adjoin_header(header);
    let body = json!({
        "repCode": "0000",
        "repData": {
            "captchaType": "blockPuzzle",
            "token": "71dd26999e314f9abb0c635336976635",
            "result": true,
            "opAdmin": false
        },
        "success": true,
        "error": false 
            });
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

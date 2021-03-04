use crate::captcha::DstDoubleBuffer;
use crate::captcha::{get_captcha, DoubleBuffer};
use crate::io::Cursor;
use base64::STANDARD;
use image::EncodableLayout;
use rocket::http::hyper::header::CONTENT_TYPE;
use rocket::http::Header;
use rocket::Response;
use rocket::{tokio::runtime::Runtime, Shutdown, State};
use std::{borrow::Borrow, cell::RefCell, collections::HashMap, rc::Rc, sync::Arc, thread};

use crate::captcha::Captcha;

use super::lib::Behavior;
use super::lib::Director;
use super::lib::D;
use std::str;
extern crate base64;

pub struct Phx {
    thread: Option<thread::JoinHandle<()>>,
    handle: Shutdown,
}
#[get("/")]
fn hello(state: State<DoubleBuffer>) -> Response {
    let mut response = Response::new();
   let header =  Header::new(CONTENT_TYPE.as_str(), "text/html");
    response.adjoin_header(header);
    match get_captcha(Arc::clone(&state)) {
        Ok(p) => {
            let image_bytes = p.dst_block.to_rgba8().into_raw();
            p.dst_block.save("a.png").unwrap();
            let rs = base64::encode(image_bytes);
            let body = format!(
                "<html>\n<body>\n<image src=\"data:image/png;base64, {}\"/>\n</body>\n</html>",
                rs
            );
            response.set_sized_body(body.len(), Cursor::new(body));
            response
        }

        Err(_) => {
            let not_found = "Not Found";
            response.set_sized_body(not_found.len(), Cursor::new(not_found));
            response
        }
    }
}

impl Phx {
    pub fn new(dst_double_buffer: DoubleBuffer) -> Self {
        let rocket = rocket::ignite()
            .mount("/", routes![hello])
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

    fn set_property(&mut self, _key: String, _property: crate::lib::Property) -> bool {
        true
    }

    fn get_property(&self, _key: &str) -> &crate::lib::Property {
        &crate::lib::Property::None
    }

    fn get_int(&self, _key: &str) -> &i32 {
        &0
    }

    fn get_float(&self, _key: &str) -> &f32 {
        &0.0
    }

    fn get_string(&self, _key: &str) -> &str {
        ""
    }

    fn name(&self) -> &str {
        ""
    }
}

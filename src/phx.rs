use crate::captcha::{DoubleBuffer, get_captcha};
use crate::captcha::DstDoubleBuffer;
use rocket::{tokio::runtime::Runtime, Shutdown, State};
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc, thread};

use crate::captcha::Captcha;

use super::lib::Behavior;
use super::lib::Director;
use super::lib::D;
pub struct Phx {
    thread: Option<thread::JoinHandle<()>>,
    handle: Shutdown
}
#[get("/")]
fn hello(state: State<DoubleBuffer>) -> &'static str {
    if let Ok(p) = get_captcha(Arc::clone(&state)){
        println!("ok...");
       p.dst_image.save("a.png") ;
       p.dst_block.save("b.png");
    }
    "Hello, world!"
}

impl Phx {
    pub fn new(dst_double_buffer: DoubleBuffer) -> Self {
        let rocket = rocket::ignite().mount("/", routes![hello]).manage(dst_double_buffer);
        let handle = rocket.shutdown();
        let thread = thread::spawn(move || {
            let r = rocket.launch();
            let rt = Runtime::new().unwrap();
            let _ = rt.block_on(r);
        });
        Phx {
            thread: Some(thread),
            handle: handle
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

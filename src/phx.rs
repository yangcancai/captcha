use rocket::{tokio::runtime::Runtime, Shutdown};
use std::thread;

use super::lib::Behavior;
pub struct Phx {
    thread: Option<thread::JoinHandle<()>>,
    handle: Shutdown,
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

impl Phx {
    pub fn new() -> Self {
        let rocket = rocket::ignite().mount("/", routes![hello]);
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

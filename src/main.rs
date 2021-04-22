use captcha::captcha::Captcha;
use captcha::captcha::DstDoubleBuffer;
use captcha::phx::Phx;
use captcha::D;
use captcha::{Actor, Behavior, Director, Man, Property};
use std::io::{self, BufRead};
use std::sync::mpsc::{self, TryRecvError};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
/// 管理模块
pub mod manager {
    use super::*;
    fn init() -> D {
        let director = Director::new();
        let mut actor = Actor::new();
        actor.set_property("name".to_string(), Property::STR("mayun".to_string()));
        actor.set_property("age".to_string(), Property::INT(32));
        actor.set_property("money".to_string(), Property::FLOAT(56666.00));

        let mut player: Actor = Actor::new();
        player.set_property("name".to_string(), Property::STR("yy".to_string()));
        player.set_property("level".to_string(), Property::INT(12));
        let mut man = Man::new("solo".to_string());
        man.prop = player;
        let dst_double_buffer = DstDoubleBuffer::new();
        let cap = Box::new(Captcha::new(Arc::clone(&dst_double_buffer)));
        director.borrow_mut().install(cap);
        let phx = Box::new(Phx::new(Arc::clone(&dst_double_buffer)));
        director.borrow_mut().install(phx);
        director.borrow_mut().init();
        director
    }

    fn terminate(director: D) {
        director.borrow_mut().terminate();
    }
    pub fn run() {
        let (tx, rx) = mpsc::channel();
        let thread = thread::spawn(move || {
            let director = init();
            loop {
                thread::sleep(Duration::from_millis(1));
                match rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
                director.borrow_mut().run();
            }
            terminate(director)
        });
        let mut line = String::new();
        let stdin = io::stdin();
        let _ = stdin.lock().read_line(&mut line);
        tx.send(()).unwrap();
        if let Ok(_) = thread.join() {};
        println!("Main thread: the associated thread was finished.");
    }
}
#[macro_use]
extern crate rocket;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
fn main() {
    manager::run();
}

extern crate image;
extern crate num_complex;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};
pub mod token;
pub mod captcha;
pub mod phx;

/// 管理模块

pub struct Plug<'a> {
    pub name: &'a str,
}

impl<'a> Task<'a, Plug<'a>> for Plug<'a> {
    fn new() -> Self {
        Plug { name: &"plug01" }
    }
    fn name(&self) -> &str {
        self.name
    }
}
pub trait Task<'a, T> {
    fn new() -> Self;
    fn install(&mut self, _plug: &'a T) -> bool {
        true
    }
    fn init(&self) -> bool {
        true
    }
    fn run(&self) -> bool {
        println!("{} run...", self.name());
        true
    }
    fn terminate(&self) -> bool {
        true
    }
    fn name(&self) -> &str;
}
/// 范型plug只能是同一类型的
pub struct Context<'a, T: Task<'a, T>> {
    plugs: HashMap<&'a str, &'a T>,
}
impl<'a, T> Task<'a, T> for Context<'a, T>
where
    T: Task<'a, T>,
{
    fn new() -> Self {
        Context {
            plugs: HashMap::new(),
        }
    }
    fn run(&self) -> bool {
        for (_, e) in &self.plugs {
            e.run();
        }
        true
    }
    fn install(&mut self, plug: &'a T) -> bool {
        self.plugs.insert(plug.name(), plug);
        true
    }
    fn name(&self) -> &str {
        "context"
    }
}

pub trait Behavior {
    fn init(&mut self) -> bool {
        true
    }
    fn run(&mut self) -> bool {
        true
    }
    fn terminate(&mut self) -> bool {
        true
    }
    fn set_property(&mut self, _key: String, _property: Property) -> bool {
        true
    }
    fn get_property(&self, _key: &str) -> &Property {
        &Property::None
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
#[derive(Debug)]
pub enum Property {
    INT(i32),
    FLOAT(f32),
    STR(String),
    None,
}

pub struct Actor {
    properties: HashMap<String, Property>,
}
pub struct Man {
    name: String,
    pub prop: Actor,
}
pub struct Director {
    pub actors: HashMap<String, Box<dyn Behavior>>,
}

impl Actor {
    pub fn new() -> Self {
        Actor {
            properties: HashMap::new(),
        }
    }
}
impl Man {
    pub fn new(name: String) -> Self {
        Man {
            name: name,
            prop: Actor::new(),
        }
    }
}
impl Behavior for Man {
    fn name(&self) -> &str {
        &self.name
    }
    fn run(&mut self) -> bool {
        println!("struct = Man, name = {}", &self.name);
        for (k, v) in &self.prop.properties {
            println!("key={:?}, val={:?}", k, v);
        }
        println!("");
        true
    }
}
impl Behavior for Actor {
    fn set_property(&mut self, key: String, property: Property) -> bool {
        self.properties.insert(key, property);
        true
    }
    fn get_property(&self, key: &str) -> &Property {
        match self.properties.get(key) {
            Some(v) => v,
            None => &Property::None,
        }
    }
    fn get_int(&self, key: &str) -> &i32 {
        match self.get_property(key) {
            Property::INT(v) => v,
            _ => &0,
        }
    }
    fn get_string(&self, key: &str) -> &str {
        match self.get_property(key) {
            Property::STR(v) => v,
            _ => "",
        }
    }
    fn get_float(&self, key: &str) -> &f32 {
        match self.get_property(key) {
            Property::FLOAT(v) => v,
            _ => &0.0,
        }
    }
    fn name(&self) -> &str {
        self.get_string("name")
    }
    fn run(&mut self) -> bool {
        for (k, v) in &self.properties {
            println!("key={:?}, val={:?}", k, v);
        }
        println!("");
        true
    }
}
pub type D = Rc<RefCell<Director>>;
impl Director {
    pub fn new() -> D {
        Rc::new(RefCell::new(Director {
            actors: HashMap::new(),
        }))
    }
    pub fn install(&mut self, actor: Box<dyn Behavior>) {
        self.actors.insert(actor.name().to_string(), actor);
    }
}
impl Behavior for Director {
    fn init(&mut self) -> bool {
        for (_, e) in self.actors.iter_mut() {
            e.init();
        }
        true
    }
    fn run(&mut self) -> bool {
        for (_, e) in self.actors.iter_mut() {
            e.run();
        }
        true
    }
    fn terminate(&mut self) -> bool {
        for (_, e) in self.actors.iter_mut() {
            e.terminate();
        }
        true
    }
    fn name(&self) -> &str {
        &""
    }
}

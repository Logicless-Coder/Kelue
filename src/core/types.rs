#[derive(Debug)]
pub enum Key {
    String(String),
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(i32),
    Float(f64),
}

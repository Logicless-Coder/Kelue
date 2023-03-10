#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Key {
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(i32),
    Float(f64),
}

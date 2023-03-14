#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum Key {
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(i32),
    Float(f64),
}

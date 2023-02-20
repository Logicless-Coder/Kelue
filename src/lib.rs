mod core;

#[cfg(test)]
mod tests {
    use crate::core::types;

    #[test]
    fn it_works() {
        let key = types::Key::String("name".to_string());
        let value1 = types::Value::String("abhishek".to_string());
        let value2 = types::Value::Number(-1);
        let value3 = types::Value::Float(2.5);

        match key {
            types::Key::String(x) => {
                assert!(x == "name");
            }
        }

        match value1 {
            types::Value::String(x) => {
                assert!(x == "abhishek");
            }
            _ => {
                assert!(false);
            }
        }

        match value2 {
            types::Value::Number(x) => {
                assert!(x == -1);
            }
            _ => {
                assert!(false);
            }
        }

        match value3 {
            types::Value::Float(x) => {
                assert!(x == 2.5);
            }
            _ => {
                assert!(false);
            }
        }
    }
}

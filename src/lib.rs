mod core;

#[cfg(test)]
mod tests {
    use crate::core::{types::{Key, Value}, store::Store, errors};

    #[test]
    fn initialize_an_empty_store() {
        let store = Store::new();
        
        assert_eq!(store.is_empty(), true);
        assert_eq!(store.size(), 0);
        assert_eq!(store.get_keys(), Vec::<&Key>::new());
    }

    #[test]
    fn writing_a_value_and_reading_it_back() {
        let mut store = Store::new();
    
        store.set(Key::String("name".to_string()), Value::String("kelue".to_string()));    
        
        match store.get(&Key::String("name".to_string())) {
            Ok(value) => assert_eq!(value, &Value::String("kelue".to_string())),
            Err(errors::KelueError::KeyNotFoundError) => panic!("This should not happen as 'name' is present in the store"),
        }
    }
}

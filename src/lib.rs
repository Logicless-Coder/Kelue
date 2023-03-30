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
        let name = String::from("name");
        let kelue = String::from("kelue");
    
        store.set(Key::String(name.clone()), Value::String(kelue.clone()));    
        
        match store.get(&Key::String(name)) {
            Ok(value) => assert_eq!(value, &Value::String(kelue)),
            Err(errors::KelueError::KeyNotFoundError) => panic!("This should not happen as 'name' is present in the store"),
        }
    }

    #[test]
    fn deleting_a_key_and_trying_to_read_it_back() {
        let mut store = Store::new();
        let name = String::from("name");
        let kelue = String::from("kelue");

        store.set(Key::String(name.clone()), Value::String(kelue));
        store.delete(&Key::String(name.clone()));

        match store.get(&Key::String(name)) {
            Ok(value) => panic!("This key should not be present in the store, value: {:?}", value),
            Err(errors::KelueError::KeyNotFoundError) => assert!(true),
        }
    }

    #[test]
    fn deleting_a_key_which_does_not_exist() {
        let mut store = Store::new();
        let name = String::from("name");

        store.delete(&Key::String(name.clone()));

        match store.get(&Key::String(name)) {
            Ok(value) => panic!("This key should not be present in the store, value: {:?}", value),
            Err(errors::KelueError::KeyNotFoundError) => assert!(true),
        }
    }
}

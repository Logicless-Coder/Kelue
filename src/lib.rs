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

        let key: Key = Key::String("name".to_string());
        let value: Value = Value::String("kelue".to_string());
    
        store.set(key.clone(), value.clone());    
        
        match store.get(&key) {
            Ok(value_found) => assert_eq!(value_found, &value),
            Err(errors::KelueError::KeyNotFoundError) => panic!("This should not happen as 'name' is present in the store"),
        }
    }

    #[test]
    fn writing_a_value_and_updating_it() {
        let mut store = Store::new();

        let key: Key = Key::String("year".to_string());
        let value: Value = Value::Number(2022);

        store.set(key.clone(), value);

        let new_value = Value::Number(2023);
        store.set(key.clone(), new_value.clone());

        match store.get(&key) {
            Ok(value_found) => assert_eq!(value_found, &new_value),
            Err(errors::KelueError::KeyNotFoundError) => panic!("This should not happen as 'year' is present in the store"),
        }
    }

    #[test]
    fn looking_for_deleted_key_should_give_error() {
        let mut store = Store::new();
        let key: Key = Key::String("year".to_string());
        let value: Value = Value::Number(2023);

        store.set(key.clone(), value.clone());
        store.erase(key.clone());

        match store.get(&key) {
            Ok(_) => panic!("This should not happen as 'year' has been DELETED from the store"),
            Err(errors::KelueError::KeyNotFoundError) => assert!(true),
        };
    }
}

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Variables {
    store: Rc<RefCell<HashMap<String, f64>>>,
}

impl Variables {
    pub fn new() -> Self {
        let mut store = HashMap::new();
        store.insert("x".to_string(), 0.0);
        store.insert("y".to_string(), 0.0);
        Self {
            store: Rc::new(RefCell::new(store)),
        }
    }

    pub fn set(&self, name: &str, value: f64) {
        self.store.borrow_mut().insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> f64 {
        *self.store.borrow().get(name).unwrap_or(&0.0)
    }
}

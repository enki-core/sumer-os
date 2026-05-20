use std::rc::Rc;
use std::cell::RefCell;
use slint::{VecModel, ModelRc};

#[derive(Clone)]
pub struct History {
    list: Rc<RefCell<Vec<slint::SharedString>>>,
}

impl History {
    pub fn new() -> Self {
        Self {
            list: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn add(&self, entry: String) -> ModelRc<slint::SharedString> {
        let mut borrowed = self.list.borrow_mut();
        borrowed.push(slint::SharedString::from(entry));
        Rc::new(VecModel::from(borrowed.clone())).into()
    }

    pub fn get_model(&self) -> ModelRc<slint::SharedString> {
        Rc::new(VecModel::from(self.list.borrow().clone())).into()
    }
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub name: String,
    pub contact_name: Option<String>,
    pub phone: Option<String>,
}

impl Customer {
    pub fn new() -> Customer {
        Customer {
            name: String::new(),
            contact_name: None,
            phone: None,
        }
    }
}
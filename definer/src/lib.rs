use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[typetag::serde(tag = "name")]
pub trait DoSomething: Debug {
    fn do_something(&self);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultImplementer1 {
    pub key1: String,
    pub key2: String,
}

#[typetag::serde(name = "default_implementer_1")]
impl DoSomething for DefaultImplementer1 {
    fn do_something(&self) {
        println!("DefaultImplementer1.do_something: {:#?}", self);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DefaultImplementer2 {
    pub key1: String,
}

#[typetag::serde(name = "default_implementer_2")]
impl DoSomething for DefaultImplementer2 {
    fn do_something(&self) {
        println!("DefaultImplementer2.do_something: {:#?}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let config = r#"
        - name: default_implementer_1
          key1: value1
          key2: value2
        - name: default_implementer_2
          key1: value1
        "#;

        let items: Vec<Box<dyn DoSomething>> = serde_yaml::from_str(config).unwrap();
        items.iter().for_each(|item| item.do_something());
    }
}

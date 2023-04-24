use definer::DoSomething;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomImplementer1 {
    pub custom_key1: String,
    pub custom_key2: String,
}

#[typetag::serde(name = "custom_implementer_1")]
impl DoSomething for CustomImplementer1 {
    fn do_something(&self) {
        println!("CustomImplementer1.do_something: {:#?}", self);
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
        - name: custom_implementer_1
          custom_key1: value1
          custom_key2: value2
        "#;

        let items: Vec<Box<dyn DoSomething>> = serde_yaml::from_str(config).unwrap();
        items.iter().for_each(|item| item.do_something());
    }
}

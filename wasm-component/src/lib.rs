wit_bindgen::generate!({
    world: "foo-world",
    path: "wit"
});

use exports::component::foo::math::{Guest, KeyValue};

struct Component;

impl Guest for Component {
    fn foo(x: i32) -> i32 {
        let y = x + 1;
        y
    }

    fn bar(keys: Vec<String>) -> Vec<KeyValue> {
        let _keys_size = keys.len();
        let _key0 = &keys[0];
        let _key1 = &keys[1];
        let _key2 = &keys[2];
        // Instead of printing to stderr, let's add a debug key-value pair
        let result: Vec<KeyValue> = keys.iter()
            .map(|key| KeyValue {
                key: key.clone(),
                value: format!("default_value_for_{}", key),
            })
            .collect();
        
        result
    }
}

export!(Component);

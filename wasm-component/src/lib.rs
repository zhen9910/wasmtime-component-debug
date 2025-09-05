wit_bindgen::generate!({
    world: "foo-world",
    path: "wit"
});

use exports::component::foo::math::Guest;

struct Component;

impl Guest for Component {
    fn foo(x: i32) -> i32 {
        let y = x + 1;
        y
    }
}

export!(Component);

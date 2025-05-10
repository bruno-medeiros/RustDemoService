pub trait FooTrait {
    fn get_name(&self) -> &str;
}

fn taking_dyn_obj(foo: &dyn FooTrait) {
    println!("{}", foo.get_name());
}

pub struct MyFoo {
    name: String,
}

impl FooTrait for MyFoo {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

#[allow(dead_code)]
pub fn test_trait() {
    let my_foo = MyFoo {
        name: "Joe".to_string(),
    };
    taking_dyn_obj(&my_foo);
}

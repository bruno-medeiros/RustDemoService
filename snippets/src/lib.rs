#![allow(clippy::disallowed_names)]

#[cfg(test)]
mod error_handling;
#[cfg(test)]
mod error_modelling_approaches;
mod lifetimes;
mod trait_objects;

mod json;
mod uri;

mod equality {
    //noinspection RsAssertEqual
    #[test]
    fn compare_strings() {
        let foo = "foo".to_string();
        assert!(foo == "foo");
        assert!(&foo == "foo");
        let foo = Some(foo);
        let foo: Option<&String> = foo.as_ref();
        assert!(foo.map(String::as_str) == Some("foo"));
    }
}

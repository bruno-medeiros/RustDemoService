#![allow(dead_code)]

// See: https://blog.rust-lang.org/2024/10/17/Rust-1.82.0.html#precise-capturing-use-syntax

use std::collections::HashMap;
use std::hash::Hash;

struct Ctx<'cx>(&'cx u8);

fn f2<'cx, 'a>(
    cx: Ctx<'cx>,
    x: &'a u8,
    // ) -> impl Iterator<Item = &'a u8> + 'cx {
) -> impl Iterator<Item = &'a u8> + use<'cx, 'a> {
    core::iter::once_with(move || {
        eprintln!("LOG: {}", cx.0);
        x
    })
    //~^ ERROR lifetime may not live long enough
}

/// Used to be an error, but is now supported.
fn non_lexical_lifetime_example() {
    fn capitalize(_data: &mut [char]) {
        // do something
    }

    let mut data = vec!['a', 'b', 'c'];
    let slice = &mut data[..]; // <-+ 'lifetime
    capitalize(slice); //   |
    data.push('d'); // ERROR!  //   |
    data.push('e'); // ERROR!  //   |
    data.push('f'); // ERROR!  //   |
}

fn test_recursive_mut() {
    let mut map = HashMap::new();
    map.insert("a", vec![1]);
    let map_borrow = &mut map;
    let x = get_default2(map_borrow, "a");
    x.push(2);

    // borrow of x is ended so we can re-use map_borrow
    map_borrow.remove("a");
    // But then this doesn't work anymore:
    //x.push(3);
}
fn get_default2<'m, K: Eq + Hash + Clone, V: Default>(
    map: &'m mut HashMap<K, V>,
    key: K,
) -> &'m mut V {
    if map.contains_key(&key) {
        // ^~~~~~~~~~~~~~~~~~ 'n
        return match map.get_mut(&key) {
            // + 'm
            Some(value) => value,   // |
            None => unreachable!(), // |
        }; // v
    }

    // At this point, `map.get_mut` was never
    // called! (As opposed to having been called,
    // but its result no longer being in use.)
    map.insert(key.clone(), V::default()); // OK now.
    map.get_mut(&key).unwrap()
}

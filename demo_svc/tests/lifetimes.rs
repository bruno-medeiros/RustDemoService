#![allow(dead_code)]


// See: https://blog.rust-lang.org/2024/10/17/Rust-1.82.0.html#precise-capturing-use-syntax

struct Ctx<'cx>(&'cx u8);

// fn f2<'cx, 'a>(
//     cx: Ctx<'cx>,
//     x: &'a u8,
// // ) -> impl Iterator<Item = &'a u8> + 'cx {
// ) -> impl Iterator<Item = &'a u8> + use<'cx, 'a> {
//     core::iter::once_with(move || {
//         eprintln!("LOG: {}", cx.0);
//         x
//     })
//     //~^ ERROR lifetime may not live long enough
// }

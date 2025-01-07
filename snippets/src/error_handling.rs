#![allow(dead_code)]

use std::error::Error;

use anyhow::anyhow;

fn anyhow_error_conversion_fail() {
    // let error: Box<dyn Error> = "xxx".to_string().into();
    // let error2: anyhow::Error = error.into();
}

fn generic_error(num: i8) -> Result<(), Box<dyn Error>> {
    if num < 0 {
        Err(format!("Num < 0: {}", num))?
    }
    anyhow_res(-1)?;

    Ok(())
}

fn anyhow_res(num: i8) -> anyhow::Result<()> {
    if num == 0 {
        anyhow::bail!("foo");
    }
    if num < 0 {
        Err(anyhow!("Num < 0: {}", num))?
    }
    Ok(())
}

use std::convert::Infallible;
pub fn unwrap_without_panic<T>(x: Result<T, Infallible>) -> T {
    let Ok(x) = x; // the `Err` case does not need to appear
    x
}

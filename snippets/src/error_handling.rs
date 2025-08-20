#![allow(dead_code)]

use std::error::Error;

fn into_std_error() -> Result<(), Box<dyn Error>> {
    let result: Result<(), Box<dyn Error>> = Err("xxx".into());
    result?;

    // alternative:
    Err("xxx")?
}
use anyhow::anyhow;

#[allow(unused_variables)]
fn anyhow_error_conversion_fail() {
    let error: Box<dyn Error> = "xxx".to_string().into();
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
        let anyhow_err: Result<(), anyhow::Error> = Err(anyhow!("Num < 0: {}", num));
        anyhow_err?
    }
    Ok(())
}

use std::convert::Infallible;

pub fn unwrap_without_panic<T>(x: Result<T, Infallible>) -> T {
    let Ok(x) = x; // the `Err` case does not need to appear
    x
}

#[cfg(test)]
mod tests {
    use anyhow::{anyhow, bail, Context};

    #[test]
    fn anyhow_display() {
        let result = foo().context("Error doing context").context("Outer ctx");

        if let Err(err) = result {
            println!("Err1: {err}");
            println!("Err2: {err:#}");
            println!("Err2: {err:?}");
        }
    }

    fn foo() -> anyhow::Result<()> {
        std::fs::read_to_string("message.txt")?;

        bail!("Inner Error")
    }

    #[test]
    fn anyhow_display2() {
        let err = anyhow!("Error1");
        let err2 = anyhow!("Error2");

        let err = err.context(err2);
        // .context("Error doing context").context("Other ctx");

        println!("Err1: {err}");
        println!("Err2: {err:#}");
        println!("Err3: {err:?}");
    }
}

#[cfg(test)]
mod error_modelling_approaches {
    use anyhow::Context;
    use thiserror::Error;

    #[tokio::test]
    async fn direct_error() {
        tracing_subscriber::fmt().try_init().ok();

        let error = reqwest::get("http://asfasfasdfsa:123").await.unwrap_err();
        println!("direct_error: {error}\n\nDebug:{error:?}\n");
    }

    // ---------- Chained error
    // One error type per layer, with a single variant each

    #[tokio::test]
    async fn chained_error() {
        let error = layer1().await.unwrap_err();

        println!("## chained_error\n{error}\n\nDebug: {error:?}");
    }

    #[derive(Debug, Error)]
    pub enum Layer1Error {
        #[error("Layer1 error: {0}")]
        Layer1Wrapper(#[from] Layer2Error),
    }

    #[derive(Debug, Error)]
    pub enum Layer2Error {
        #[error("Layer2 error: {0}")]
        Layer2Wrapper(#[from] reqwest::Error),
    }

    pub async fn layer1() -> Result<(), Layer1Error> {
        let _calc_a = layer2().await?;
        // ...
        let _calc_b = layer2().await?;
        Ok(())
    }

    pub async fn layer2() -> Result<(), Layer2Error> {
        reqwest::get("http://asfasfasdfsa:123").await?;
        Ok(())
    }

    // ---------- Chained Error - Fine Grained
    // One error type per layer, with multiple variants in each error

    #[tokio::test]
    async fn chained_error_fine_grained() {
        let error = layer1_fine_grained().await.unwrap_err();

        println!("## layer1_fine_grained\n{error}\n\nDebug: {error:?}");
    }

    pub async fn layer1_fine_grained() -> Result<(), Layer1ErrorFineGrained> {
        let _calc_foo = layer2()
            .await
            .map_err(Layer1ErrorFineGrained::Layer1WrapperCallFoo)?;
        // ...
        let _calc_bar = layer2()
            .await
            .map_err(Layer1ErrorFineGrained::Layer1WrapperCallBar)?;
        Ok(())
    }

    #[derive(Debug, Error)]
    pub enum Layer1ErrorFineGrained {
        #[error("Layer1 Foo error: {0}")]
        Layer1WrapperCallFoo(#[source] Layer2Error),
        #[error("Layer1 Bar error: {0}")]
        Layer1WrapperCallBar(#[source] Layer2Error),
        #[error("Layer1 Xpto error: {0}")]
        Layer1WrapperCallXpto(#[source] Layer2Error),
    }

    // ---------- Chained Error - Semi Fine Grained
    // One error type per layer, with a single variant,
    // but the variant has a context String to distinguish different instances in code.

    #[derive(Debug, Error)]
    pub enum Layer1ErrorWithContext {
        #[error("Layer1 {0} error: {1}")]
        Layer1WrapperCall(String, #[source] Layer2Error),
    }

    pub async fn layer1_fine_grained_with_context() -> Result<(), Layer1ErrorWithContext> {
        let _calc_foo = layer2()
            .await
            .map_err(|e| Layer1ErrorWithContext::Layer1WrapperCall("Foo call".to_string(), e))?;
        // ...
        let _calc_bar = layer2()
            .await
            .map_err(|e| Layer1ErrorWithContext::Layer1WrapperCall("Bar call".to_string(), e))?;
        Ok(())
    }

    // ---------- Chained Error - Using anyhow error, and `.context()` for context info

    #[tokio::test]
    async fn chained_error_anyhow() {
        let error = layer1_anyhow().await.unwrap_err();

        println!("## chained_error_anyhow\n{error}\n\nDebug: {error:?}\n\n---");
    }

    pub async fn layer1_anyhow() -> Result<(), anyhow::Error> {
        let _calc_foo = layer2_anyhow().await.context("Foo Call")?;
        // ...
        let _calc_bar = layer2_anyhow().await.context("Bar Call")?;
        Ok(())
    }

    pub async fn layer2_anyhow() -> Result<(), anyhow::Error> {
        reqwest::get("http://asfasfasdfsa:123").await?;
        Ok(())
    }
}

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
use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("HTTP transport error: {0} -- {0:#} -- {0:?}")]
    Transport(#[from] reqwest::Error),

    #[error("Step '{step_name}' execution failed for workflow run. {source}")]
    StepExecutionFailed {
        step_name: String,
        #[source]
        source: anyhow::Error,
    },
}

mod test2 {
    use std::error::Error;

    use crate::error_handling::HttpClientError;

    #[tokio::test]
    async fn test_display_of_structured_error() {
        reqwest::get("http://asfasfasdfsadfasdfs:123")
            .await
            .map_err(|err| {
                println!("Err1: {err}");
                println!("Err2: {err:#}");
                println!("Err3: {err:?}");

                if let Some(source_err) = err.source() {
                    println!(" Source1: {:?}", source_err);
                }
            })
            .ok();

        foo()
            .await
            .map_err(|err| {
                println!("Err1: {err}");
                println!("Err2: {err:#}");
                println!("Err3: {err:?}");
            })
            .ok();
    }

    pub async fn foo() -> Result<(), HttpClientError> {
        reqwest::get("http://asfasfasdfsadfasdfs:123").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_display_of_structured_error2() {
        foo()
            .await
            .map_err(|err| {
                let err = HttpClientError::StepExecutionFailed {
                    step_name: "Step".into(),
                    source: err.into(),
                };
                println!("Err1: {err}");
                println!("Err2: {err:#}");
                println!("Err3: {err:?}");
            })
            .ok();
    }
}

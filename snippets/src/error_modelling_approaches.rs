use anyhow::{Context, Error};
use thiserror::Error;
use tracing::info;

#[tokio::test]
async fn direct_error() {
    let error = reqwest::get("http://asfasfasdfsa:123").await.unwrap_err();
    println!("Display: {error}\n");
    println!("Debug: {error:?}\n");
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

pub async fn layer1() -> Result<u32, Layer1Error> {
    let calc_foo = layer2().await?;
    // ...
    let calc_bar = layer2().await?;
    Ok(calc_foo + calc_bar)
}

pub async fn layer2() -> Result<u32, Layer2Error> {
    reqwest::get("http://asfasfasdfsa:123").await?;
    Ok(123)
}

// ---------- Chained Error - Fine Grained
// One error type per layer, with multiple variants in each error

#[tokio::test]
async fn chained_error_fine_grained() {
    let error = layer1_fine_grained().await.unwrap_err();

    println!("## layer1_fine_grained\n{error}\n\nDebug: {error:?}");
}

pub async fn layer1_fine_grained() -> Result<u32, Layer1ErrorFineGrained> {
    let calc_foo = layer2()
        .await
        .map_err(Layer1ErrorFineGrained::Layer1WrapperCallFoo)?;
    // ...
    let calc_bar = layer2()
        .await
        .map_err(Layer1ErrorFineGrained::Layer1WrapperCallBar)?;
    Ok(calc_foo + calc_bar)
}

#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
pub enum Layer1ErrorFineGrained {
    #[error("Layer1 Foo error: {0}")]
    Layer1WrapperCallFoo(#[source] Layer2Error),
    #[error("Layer1 Bar error: {0}")]
    Layer1WrapperCallBar(#[source] Layer2Error),
    #[error("Layer1 Xpto error: {0}")]
    #[allow(unused)]
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

#[tokio::test]
async fn chained_error_with_context() {
    let error = layer1_fine_grained_with_context().await.unwrap_err();

    println!("## layer1_fine_grained\n{error}\n\nDebug: {error:?}");
}

pub async fn layer1_fine_grained_with_context() -> Result<u32, Layer1ErrorWithContext> {
    let calc_foo = layer2()
        .await
        .map_err(|e| Layer1ErrorWithContext::Layer1WrapperCall("Foo call".to_string(), e))?;
    // ...
    let calc_bar = layer2()
        .await
        .map_err(|e| Layer1ErrorWithContext::Layer1WrapperCall("Bar call".to_string(), e))?;
    Ok(calc_foo + calc_bar)
}

// ---------- Chained Error - Using anyhow error, and `.context()` for context info

#[tokio::test]
async fn chained_error_anyhow() {
    tracing_subscriber::fmt().try_init().ok();

    let error: Error = layer1_anyhow().await.unwrap_err();

    println!("## chained_error_anyhow");
    println!("{error}");
    println!("Debug: {error:?}\n---");

    // tracing::error!(error = &error as &(dyn std::error::Error));
    tracing::error!(error = &*error as &dyn std::error::Error);

    info!(error = %error, "%Error:");
    info!(error = ?error, "?Error:");
}

pub async fn layer1_anyhow() -> Result<u32, anyhow::Error> {
    let calc_foo = layer2_anyhow().await.context("Foo Call")?;
    // ...
    let calc_bar = layer2_anyhow().await.context("Bar Call")?;
    Ok(calc_foo + calc_bar)
}

pub async fn layer2_anyhow() -> Result<u32, anyhow::Error> {
    reqwest::get("http://asfasfasdfsa:123").await?;
    Ok(123)
}

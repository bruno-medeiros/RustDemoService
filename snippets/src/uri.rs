#![cfg(test)]

use http::uri::Uri;
use url::Url;

#[test]
fn valid_urls() -> anyhow::Result<()> {
    Url::parse("s://foo.com/path/")?;
    Uri::try_from("s://foo.com/path/")?;

    Url::parse("s:foo.com/path/")?;
    Uri::try_from("s:foo.com/path/").unwrap_err();

    Url::parse("/foo.com/path").unwrap_err();
    let uri = Uri::try_from("/foo.com/path")?; // Why is this valid?
    assert_eq!(uri.scheme(), None);

    Url::parse("foo.com/path").unwrap_err();
    Uri::try_from("foo.com/path").unwrap_err();
    // assert!(uri.)

    Ok(())
}
#[test]
fn compare_urls() -> anyhow::Result<()> {
    let url_str = "https://github.com/rust-lang//rust/issues?labels=E-easy&state=open";

    let url = Url::parse(url_str)?;
    assert_eq!(url.path(), "/rust-lang//rust/issues");

    let uri = Uri::try_from(url_str)?;
    assert_eq!(uri.path(), "/rust-lang//rust/issues");

    assert_ne!(
        Url::parse("s://foo.com/path/")?,
        Url::parse("s://foo.com/path")?
    );

    Ok(())
}

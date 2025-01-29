use http::uri::Uri;
use url::Url;

#[test]
fn compare_urls() -> anyhow::Result<()> {
    let url_str = "https://github.com/rust-lang//rust/issues?labels=E-easy&state=open";

    let url = Url::parse(url_str)?;
    assert_eq!(url.path(), "/rust-lang//rust/issues");

    let uri = Uri::try_from(url_str)?;
    assert_eq!(uri.path(), "/rust-lang//rust/issues");


    assert_eq!(Url::parse("foo.com/path/")?, Url::parse("foo.com/path")?);
    assert_eq!(Uri::try_from("foo.com/path/")?, Uri::try_from("foo.com/path")?);

    Ok(())
}

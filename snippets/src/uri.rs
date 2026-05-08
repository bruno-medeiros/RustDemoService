#![cfg(test)]

use http::uri::Uri;
use url::Url;

#[test]
fn valid_urls() {
    Url::parse("s://foo.com/path/").unwrap();
    Uri::try_from("s://foo.com/path/").unwrap();

    Url::parse("s:foo.com/path/").unwrap();
    Uri::try_from("s:foo.com/path/").unwrap_err();

    Url::parse("/foo.com/path").unwrap_err();
    let uri = Uri::try_from("/foo.com/path").unwrap(); // Why is this valid.unwrap()
    assert_eq!(uri.scheme(), None);

    Url::parse("foo.com/path").unwrap_err();
    Uri::try_from("foo.com/path").unwrap_err();
}
#[test]
fn compare_urls() {
    let url_str = "https://github.com/rust-lang//rust/issues.unwrap()labels=E-easy&state=open";

    let url = Url::parse(url_str).unwrap();
    assert_eq!(url.path(), "/rust-lang//rust/issues");

    let uri = Uri::try_from(url_str).unwrap();
    assert_eq!(uri.path(), "/rust-lang//rust/issues");

    assert_ne!(
        Url::parse("s://foo.com/path/").unwrap(),
        Url::parse("s://foo.com/path").unwrap()
    );
}

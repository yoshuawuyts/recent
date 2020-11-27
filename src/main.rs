//! GitHub issue redirect service

#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style, rust_2018_idioms)]

use chrono::Duration;
use tide::Status;
use tide::{http::Url, Request};

#[async_std::main]
async fn main() -> std::io::Result<()> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/").get(index);
    app.at("/:username/issues/weeks/:weeks").get(issues);
    app.at("/:username/pulls/weeks/:weeks").get(pulls);
    app.listen("localhost:8080").await?;
    Ok(())
}

pub async fn index(_req: Request<()>) -> tide::Result {
    let body = r#"
        <body>
            <h1>Welcome to GitHub redirect</h1>
            <header>
                This page provides permalinks to GitHub queries. Because GitHub only supports
                absolute URLs in their issue search. So we've built a page that enables permalinks
                against <em>relative</em> queries.
            </header>
            <main>
                <h2>Examples</h2>
                <p>Visit <a
                href="/http-rs/issues/weeks/1">/http-rs/issues/weeks/1</a> to
                see all of the http-rs issues that have seen activity in the
                past week.<p>

                <p>Visit <a
                href="/http-rs/pulls/weeks/1">/http-rs/pulls/weeks/1</a> to
                see all of the http-rs pull-requests that have seen activity
                in the past week.</p>
            </main>
        </body>
    "#;
    let res = tide::Response::builder(200)
        .content_type(tide::http::mime::HTML)
        .body(html_index::Builder::new().raw_body(body).build())
        .build();
    Ok(res)
}

pub async fn issues(req: Request<()>) -> tide::Result {
    let username = req.param("username").unwrap();
    let weeks = req.param("weeks").unwrap();

    let weeks = Duration::weeks(weeks.parse().status(400)?);
    let now = chrono::offset::Utc::now();
    let date = now - weeks;
    let date = date.to_rfc3339();
    let date = date.split("T").next().unwrap();

    let mut url = Url::parse("https://github.com/issues").unwrap();
    let pairs = [
        "user:USERNAME",
        "is:open",
        "is:pr",
        "archived:false",
        "no:label",
        "sort:updated-desc",
        "-updated:<DATE",
    ]
    .join("+")
    .replace("USERNAME", username)
    .replace("DATE", date);
    url.set_query(Some(format!("q={}", pairs).as_str()));
    Ok(tide::Redirect::new(url.as_str()).into())
}

pub async fn pulls(req: Request<()>) -> tide::Result {
    let username = req.param("username").unwrap();
    let weeks = req.param("weeks").unwrap();

    let weeks = Duration::weeks(weeks.parse().status(400)?);
    let now = chrono::offset::Utc::now();
    let date = now - weeks;
    let date = date.to_rfc3339();
    let date = date.split("T").next().unwrap();

    let mut url = Url::parse("https://github.com/issues").unwrap();
    let pairs = [
        "user:USERNAME",
        "is:open",
        "is:pr",
        "archived:false",
        "no:label",
        "sort:updated-desc",
        "-updated:<DATE",
    ]
    .join("+")
    .replace("USERNAME", username)
    .replace("DATE", date);
    url.set_query(Some(format!("q={}", pairs).as_str()));
    Ok(tide::Redirect::new(url.as_str()).into())
}

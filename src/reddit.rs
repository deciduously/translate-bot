extern crate serde_json;

use std::{env, str};
use futures::{Future, Stream};
use hyper::{Client, Method, Request};
use hyper::header::{Authorization, Basic, ContentLength, ContentType, UserAgent};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

const REDDIT_URI: &str = "reddit.com/api/v1";

#[derive(Deserialize)]
struct TokenRes {
    access_token: String,
    expires_in: u64,
    scope: String,
    token_type: String,
}

//retrieves access token
pub fn request_token() -> Result<String, Box<::std::error::Error>> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    let data = format!(
        "grant_type=password&username=IKnowYouDidntAskBut&password={}",
        env::var("REDDIT_PASSWORD").unwrap()
    );
    let uri = format!("https://www.{}/access_token", REDDIT_URI).parse()?;
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::form_url_encoded());
    req.headers_mut().set(ContentLength(data.len() as u64));
    req.headers_mut().set(UserAgent::new(
        "linux:hyper:0.11 (by /u/IKnowYouDidntAskBut)",
    ));
    req.headers_mut().set(Authorization(Basic {
        username: env::var("REDDIT_ID").unwrap(),
        password: Some(env::var("REDDIT_SECRET").unwrap()),
    }));
    req.set_body(data);

    let post = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());
        res.body().concat2().and_then(move |body| {
            let v: TokenRes = serde_json::from_slice(&body).unwrap();
            Ok(v.access_token)
        })
    });
    let token = core.run(post).unwrap();
    Ok(token)
}

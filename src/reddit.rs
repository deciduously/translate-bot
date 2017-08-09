extern crate serde_json;

use std::{env, str};
use std::str::FromStr;
use futures::{Future, Stream};
use hyper::{Client, Method, Request};
use hyper::header::{Authorization, Basic, Bearer, ContentLength, ContentType, UserAgent};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

const REDDIT_URI: &str = "reddit.com";
const USER_AGENT: &str = "linux:hyper:0.11 (by /u/IKnowYouDidntAskBut";

#[derive(Deserialize)]
struct TokenRes {
    access_token: String,
    expires_in: u64,
    scope: String,
    token_type: String,
}

//token_request builds the hyper::Request for obtaining the oauth token
fn token_request() -> Result<Request, Box<::std::error::Error>> {
    let data = format!(
        "grant_type=password&username=IKnowYouDidntAskBut&password={}",
        env::var("REDDIT_PASSWORD").unwrap()
    );
    let uri = format!("https://www.{}/api/v1/access_token", REDDIT_URI).parse()?;
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::form_url_encoded());
    req.headers_mut().set(ContentLength(data.len() as u64));
    req.headers_mut().set(UserAgent::new(USER_AGENT));
    req.headers_mut().set(Authorization(Basic {
        username: env::var("REDDIT_ID").unwrap(),
        password: Some(env::var("REDDIT_SECRET").unwrap()),
    }));
    req.set_body(data);
    Ok(req)
}

//oauth_request builds the hyper::Request for specified endpoint
fn oauth_request(e: &str, t: &str) -> Result<Request, Box<::std::error::Error>> {
    let uri = format!("https://oauth.{}/{}", REDDIT_URI, e).parse()?;
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(UserAgent::new(USER_AGENT));
    req.headers_mut().set(Authorization(Bearer::from_str(t).unwrap()));
    Ok(req)
}

//retrieves access token
pub fn request_me() -> Result<String, Box<::std::error::Error>> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    let post = client.request(token_request()?).and_then(|res| {
        println!("POST: {}", res.status());
        res.body().concat2().and_then(move |body| {
            let v: TokenRes = serde_json::from_slice(&body).unwrap();
            Ok(v.access_token)
        })
    });
    let token = core.run(post).unwrap();

    let work = client.request(oauth_request("api/v1/me", &token)?).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
            Ok(v)
        })
    });
    let me = core.run(work).unwrap();
    println!("{:?}", me);

    Ok("Success".to_owned())
}

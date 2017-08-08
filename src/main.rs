extern crate dotenv;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio_core;

use dotenv::dotenv;
use std::{env, str};
use futures::{Future, Stream};
use hyper::{Client, Method, Request};
use hyper::header::{ContentLength, ContentType};
use hyper_tls::HttpsConnector;
use serde_json::Value;
use tokio_core::reactor::Core;

//use rawr::prelude::*;
//use rawr::traits::{Commentable, Content};

const TRANSLATE_URI: &str = "https://translation.googleapis.com/language/translate/v2";

//TODO NO CRATE FOR YOU.  Write it your damn self.
/**
//grab_post grabs the post to translate
fn grab_post() -> String {
    let client = RedditClient::new(
        "linux:rawr:0.1.1 (by /u/IKnowYouDidntAskBut)",
        AnonymousAuthenticator::new(),
    );
    let all = client.subreddit("all");
    let mut rising_all = all.rising(ListingOptions::default()).expect(
        "Could not fetch post listing!",
    );

    String::from(
        rising_all
            .next()
            .expect("Could not grab post")
            .replies()
            .expect("Could not get replies")
            .next()
            .expect("Could not grab top")
            .body()
            .expect("Could not grab body"),
    )
}
*/

//translate takes a string and returns a translated string
//TODO Box<Error> is not ideal - learn how to actually handle errors
fn translate() -> Result<String, Box<::std::error::Error>> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    //TODO get Serde to do this
    let json = r#"{"q":"translate this string","source":"en","target":"es","format":"text"}"#;

    let uri = format!("{}?key={}", TRANSLATE_URI, env::var("KEY").unwrap()).parse()?;
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);

    let post = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());
        res.body().concat2().and_then(move |body| {
            //TODO START HERE use stronger Serde
            let v: Value = serde_json::from_slice(&body).unwrap();
            println!("Translated string: {}", v["data"]);
            Ok(())
        })
    });

    core.run(post)?;
    //println!("POST RESP: {}", str::from_utf8(&posted)?);
    Ok(String::from("Hey!"))
}

fn main() {
    dotenv().ok();
    //let body = grab_post();
    //println!("{}", body);
    println!("{:?}", translate());
}

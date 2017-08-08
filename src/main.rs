extern crate dotenv;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;

use dotenv::dotenv;
use std::{env, str};
use futures::{Future, Stream};
use hyper::{Client, Method, Request};
use hyper::header::{ContentLength, ContentType};
use hyper_tls::HttpsConnector;
use rand::Rng;
use serde_json::{Error, Value};
use tokio_core::reactor::Core;

const TRANSLATE_URI: &str = "https://translation.googleapis.com/language/translate/v2";

#[derive(Clone, Copy)]
enum Lang {
    En,
    Es,
    Ru,
    Fr,
}

#[derive(Serialize)]
struct TranslateReq {
    q: String,
    source: String,
    target: String,
    format: String,
}

#[derive(Debug, Deserialize)]
struct TranslateRes {
    data: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    translations: Vec<Translations>,
}

#[derive(Debug, Deserialize)]
struct Translations {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

fn create_translate_req(s: &str, l: &Lang) -> Result<String, Error> {
    let ret = TranslateReq {
        q: s.to_owned(),
        source: "en".to_owned(),
        target: match *l {
            Lang::En => "en".to_owned(),
            Lang::Es => "es".to_owned(),
            Lang::Ru => "ru".to_owned(),
            Lang::Fr => "fr".to_owned(),
        },
        format: "text".to_owned(),
    };

    serde_json::to_string(&ret)
}

fn random_lang() -> Lang {
    //all but En
    let langs = vec![Lang::Es, Lang::Fr, Lang::Ru];

    *rand::thread_rng().choose(&langs).unwrap()
}

//translate takes a string and returns a translated string
//TODO Box<Error> is not ideal - learn how to actually handle errors
fn translate(s: &str, l: &Lang) -> Result<String, Box<::std::error::Error>> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    //TODO random language??
    let json = create_translate_req(s, l)?;

    let uri = format!("{}?key={}", TRANSLATE_URI, env::var("KEY").unwrap())
        .parse()?;
    let mut req = Request::new(Method::Post, uri);
    req.headers_mut().set(ContentType::json());
    req.headers_mut().set(ContentLength(json.len() as u64));
    req.set_body(json);

    let post = client.request(req).and_then(|res| {
        println!("POST: {}", res.status());
        res.body().concat2().and_then(move |body| {
            let v: TranslateRes = serde_json::from_slice(&body).unwrap();
            Ok((v.data.translations[0].translated_text.clone()))
        })
    });

    Ok(core.run(post).unwrap())
}

fn main() {
    dotenv().ok();

    println!("{}", translate("I am a book", &random_lang()).unwrap());
}

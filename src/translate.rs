extern crate serde_json;

use lang::Lang;
use std::{env, str};
use futures::{Future, Stream};
use hyper::{Client, Method, Request};
use hyper::header::{ContentLength, ContentType};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

const TRANSLATE_URI: &str = "https://translation.googleapis.com/language/translate/v2";

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

fn create_translate_req(s: &str, l: &Lang) -> Result<String, serde_json::Error> {
    let ret = TranslateReq {
        q: s.to_owned(),
        source: "en".to_owned(),
        target: l.short(),
        format: "text".to_owned(),
    };

    serde_json::to_string(&ret)
}

//translate takes a string and returns a translated string
//TODO Box<Error> is not ideal - learn how to actually handle errors
pub fn translate(s: &str, l: &Lang) -> Result<String, Box<::std::error::Error>> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

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
            Ok(v.data.translations[0].translated_text.clone())
        })
    });

    Ok(core.run(post).unwrap())
}

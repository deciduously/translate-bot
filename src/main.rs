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

mod lang;
mod translate;

use dotenv::dotenv;
use lang::Lang;
use translate::translate;

fn main() {
    dotenv().ok();

    let en_text = "This language is cool";
    let lang = Lang::random();
    println!(
        "'{}' is '{}' in {}",
        en_text,
        translate(en_text, &lang).unwrap(),
        lang
    );
}

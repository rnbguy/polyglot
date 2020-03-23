use clap::{App, Arg};

mod deepl;

fn main() {
    let matches = App::new("polyglot")
        .about("Translate texts")
        .arg(
            Arg::with_name("source")
                .short("s")
                .long("source")
                .value_name("SOURCE")
                .help("Source language code")
                .default_value("auto"),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("TARGET")
                .help("Target language code")
                .required(true),
        )
        .arg(
            Arg::with_name("text")
                .value_name("TXT")
                .help("Text to translate")
                .required(true),
        )
        .get_matches();

    let text = matches.value_of("text").unwrap();
    let source = matches.value_of("source").unwrap();
    let target = matches.value_of("target").unwrap();

    let payload = deepl::Payload::new(text.into(), source.into(), target.into());

    let url = "https://www2.deepl.com/jsonrpc";

    let client = reqwest::blocking::Client::new();
    let resp: serde_json::Value = client
        .post(url)
        .body(serde_json::to_string_pretty(&payload).unwrap())
        .send()
        .unwrap()
        .json()
        .unwrap();

    println!("{}", serde_json::to_string_pretty(&resp).unwrap());
}

extern crate regex;
use regex::Regex;
extern crate hyper;
use hyper::Client;
#[macro_use]
extern crate lazy_static;
use std::io::prelude::*;

lazy_static! {
    static ref SECTION: Regex = Regex::new(r#"\{"data":\[([\s\S]+?)\],"error":""\}"#).unwrap();
    static ref ERR: Regex = Regex::new(r#"\{"data":\[\],"error":"NO such list found"\}"#).unwrap();
}

fn build_url(url: &str, c: char, len: usize) -> String {
    let mut x = url.to_string();
    x.push(c);
    x.push_str("&len=");
    x.push_str(&len.to_string());
    x
}

fn main() {
    //get base url
    let base = "http://scrabble.merriam.com/lapi/1/sbl_finder/get_limited_data?mode=wfd&type=begins&rack=";
    let client = Client::new();
    //list all english letters
    let data: Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let lens: Vec<usize> = vec![2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    //build an HTTP client
    let mut client = Client::new();
    //empty vector to store all words
    let mut output = String::with_capacity(10000000);
    //loop over all letter
    for c in data {
        //loop over lengths
        for l in lens.iter() {
            //request data
            let url = build_url(base,c.clone(),l.clone());
            let mut res = match client.get(&url).send() {
                Ok(x) => x,
                Err(e) => panic!("Panic on request {:?}", e)
            };
            //read packet
            let mut body = String::with_capacity(8000);
            match res.read_to_string(&mut body) {
                Ok(_) => { },
                Err(e) => panic!("Failed to read response {:?}", e)
            };
            //parse outbody
            let data = match SECTION.captures(&body) {
                Option::Some(x) => x.at(1).unwrap(),
                Option::None => if ERR.is_match(&body) {
                    break;
                } else {
                    panic!("\n\nNew failure!!\n\n{:?}\n\n", &body);
                }
            };
            //temp buffer
            let mut temp = String::with_capacity(50);
            //parse body
            for c in data.chars() {
                match c {
                    '"' => continue,
                    ',' => {
                        output.push_str(&temp);
                        output.push('\n');
                        temp.clear();
                    },
                    x if x <= 'z' && x >= 'a' => temp.push(x),
                    _ => continue
                };
            }
            output.push_str(&temp);
            output.push('\n');
            temp.clear();
        }
    }
    println!("{}", output);
}

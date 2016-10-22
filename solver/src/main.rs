
use std::io::prelude::*;
use std::fs::File;
use std::cmp::Ordering;


fn continuation(new: &str, old: &str) -> bool {
    (new.len() == (old.len() + 1))
    &&
    (
        new.chars().skip(1).zip(old.chars()).fold(true,|x,y|x&&(y.0==y.1))
        ||
        old.chars().zip(new.chars()).fold(true,|x,y|x&&(y.0==y.1))
    )
}

fn recurse<'a>(word: &'a str, stack: &mut Vec<&'a str>, dict: &Vec<&'a str>, max: &mut usize) {
    //build lambda to filer items
    let lambda = |x: &&&str| -> bool {
        continuation(x, word)
    };
    //get list of new items to filer
    let new_items: Vec<&&'a str> = dict.iter().filter(lambda).collect();
    //this branch is done!
    if new_items.is_empty() && stack.len() >= *max {
        //print new information
        *max = stack.len();
        println!("New max! {}", *max);
        for item in stack.iter() {
            println!("{}", item);
        }
        println!("");
        return;
    }
    //continue
    for new_word in new_items {
        stack.push(new_word);
        recurse(new_word,stack,dict,max);
        stack.pop();
    }
}



//
//ENTRY POINT
//
fn main() {
    //read cli argument
    let args: Vec<String> = ::std::env::args().skip(1).collect();
    //open first file as argument
    let mut input = match File::open(&args[0]) {
        Ok(x) => x,
        Err(e) => panic!("\n\nCould not open file\n\n{:?}\n\n", e)
    };
    //build buffer to story dictionary in (10Mb)
    let mut body = String::with_capacity(10000000);
    //read file
    match input.read_to_string(&mut body) {
        Ok(_) => { },
        Err(e) => panic!("\n\nCould not read file\n\n{:?}\n\n",e)
    };
    //split file into individual words
    let mut v = Vec::<&str>::with_capacity(200000);
    for word in body.lines() {
        v.push(word);
    }
    let mut max = 0usize;
    //get all 2 length words
    for word_len2 in v.iter().filter(|x| x.len() == 2) {
        let mut stack = Vec::<&str>::new();
        stack.push(word_len2);
        recurse(word_len2,&mut stack,&v,&mut max);
    }
}

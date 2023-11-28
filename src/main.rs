#[warn(unused_variables)]
#[warn(dead_code)]

use std::env;
use std::fs;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum TokenTypes {
  TokRParen,
  TokLParen,
  TokEqual,
  TokNotEqual,
  TokGreater,
  TokLess,
  TokGreaterEqual,
  TokLessEqual,
  TokOr,
  TokAnd,
  TokNot,
  TokIf,
  TokThen,
  TokElse,
  TokAdd,
  TokSub,
  TokMult,
  TokDiv,
  TokConcat,
  TokLet,
  TokRec,
  TokIn,
  TokDef,
  TokFun,
  TokArrow,
  TokInt(u64),
  TokBool(bool),
  TokString(String),
  TokID(String),
  TokDoubleSemi
}


// Define a static regex
lazy_static! {
    static ref RE_WHITESPACE:Regex = Regex::new(r"^[\n\t\r ]+").unwrap();
    static ref RE_STRING:Regex = Regex::new(r#".+\"#).unwrap();
    static ref RE_LPAREN:Regex  = Regex::new(r"^\(").unwrap();
    static ref RE_RPAREN:Regex = Regex::new(r"^\)").unwrap();
    static ref RE_EQUAL:Regex = Regex::new(r"^=").unwrap();
    static ref RE_NOTEQUAL:Regex = Regex::new(r"^<>").unwrap();
    static ref RE_GREATER:Regex = Regex::new(r"^>").unwrap();
    static ref RE_LESSER:Regex= Regex::new(r"^<").unwrap();
    static ref RE_LESSEQUAL:Regex = Regex::new(r"^<=").unwrap();
    static ref RE_GREATEREQUAL:Regex = Regex::new(r"^>=").unwrap();
    static ref RE_OR:Regex = Regex::new(r"^||").unwrap();
    static ref RE_AND:Regex = Regex::new(r"^&&").unwrap();
    static ref RE_NOT:Regex = Regex::new(r"^not").unwrap();
    static ref RE_IF:Regex = Regex::new(r"^if").unwrap();
    static ref RE_THEN:Regex = Regex::new(r"^then").unwrap();
    static ref RE_ELSE:Regex = Regex::new(r"^else").unwrap();
    static ref RE_ADD:Regex = Regex::new(r"^\+").unwrap();
    static ref RE_SUB:Regex = Regex::new(r"^-").unwrap();
    static ref RE_MULT:Regex = Regex::new(r"^\*").unwrap();
    static ref RE_DIV:Regex = Regex::new(r"^\/").unwrap();
    static ref RE_CONCAT:Regex = Regex::new(r"^\^").unwrap();
    static ref RE_LET:Regex = Regex::new(r"^let").unwrap();
    static ref RE_REC:Regex = Regex::new(r"^rec").unwrap();
    static ref RE_IN:Regex = Regex::new(r"^in").unwrap();
    static ref RE_DEF:Regex = Regex::new(r"^def").unwrap();
    static ref RE_FUN:Regex = Regex::new(r"^fun").unwrap();
    static ref RE_ARROW:Regex = Regex::new(r"^->").unwrap();
    static ref RE_PINT:Regex = Regex::new(r"^[0-9]+").unwrap();
    static ref RE_NINT:Regex = Regex::new(r"^-[0-9]+").unwrap();
    static ref RE_BOOL:Regex = Regex::new(r"^(false|true)").unwrap();
    static ref RE_ID:Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap();
    static ref RE_DOUBLESEMI:Regex = Regex::new(r";;").unwrap();
}

fn tokenizer(content:String, position:u32, file_size:u32)-> Vec<TokenTypes>{
    
    let tokens :Vec<TokenTypes> = Vec::new();
    // if position < file_size {
    //     if let Some(matched) = RE_WHITESPACE.find(&content[position..]){
    //         print!("");
    //     }
    //     //     println!("matched position {}", matched.start());
    //     // } else if let Some(matched)=RE_{
    //     //     
    //     // }   
    // }

    tokens
}

fn main() {

    if let Some(matched) = RE_ID.find(" letall this be what we need"){
        println!("matched position {}", matched.start());
        println!("matched position {}", matched.end());
        println!("matched position {}", matched.as_str());
    } 
    return;
    let args: Vec<String> = env::args().collect();
     // dbg!(args);
    let filename = env::args().nth(1).expect("no filename given");

    if args.len() > 1{

        let contents = fs::read_to_string(&filename)
            .expect(&filename);

        // println!("Source code:\n{contents}");
        //
        // print!("the length of the content {}", contents.len());
        tokenizer(contents, 0, 0);
        // println!("{:?}", tokenizer(contents))
    }   
}


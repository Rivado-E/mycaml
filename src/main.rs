#![allow(unused_variables)]
#![allow(dead_code)]

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
  TokInt(i64),
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
    static ref RE_PINT:Regex = Regex::new(r"^-?\d+").unwrap();
    static ref RE_NINT:Regex = Regex::new(r"^-[0-9]+").unwrap();
    static ref RE_BOOL:Regex = Regex::new(r"^(false|true)").unwrap();
    static ref RE_ID:Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap();
    static ref RE_DOUBLESEMI:Regex = Regex::new(r";;").unwrap();
}

fn tokenize(content: &String)-> Vec<TokenTypes>{
    println!("Tokenize is called with {}", content); 
    let mut tokens :Vec<TokenTypes> = Vec::new();
    let mut pos: usize = 0;
    let end:usize = content.len();

    while pos < end {
        println!("pos at {}", pos);
        if let Some(matched) = RE_WHITESPACE.find(&content[pos..]){
            pos += matched.len();
        }
        else if let Some(matched) = RE_EQUAL.find(&content[pos..]){
            tokens.push(TokenTypes::TokEqual);
            pos += matched.len();
        }
        else if let Some(matched) = RE_NOTEQUAL.find(&content[pos..]){
            tokens.push(TokenTypes::TokNotEqual);
            pos += matched.len();
        }
        else if let Some(matched) = RE_LESSEQUAL.find(&content[pos..]){
            tokens.push(TokenTypes::TokLessEqual);
            pos += matched.len();
        }
        else if let Some(matched) = RE_GREATER.find(&content[pos..]){
            tokens.push(TokenTypes::TokGreater);
            pos += matched.len();
        }
        else if let Some(matched) = RE_LESSER.find(&content[pos..]){
            tokens.push(TokenTypes::TokLess);
            pos += matched.len();
        }
        else if let Some(matched) = RE_AND.find(&content[pos..]){
            tokens.push(TokenTypes::TokAnd);
            pos += matched.len();
        }
        else if let Some(matched) = RE_OR.find(&content[pos..]){
            tokens.push(TokenTypes::TokOr);
            pos += matched.len();
        }
        else if let Some(matched) = RE_ADD.find(&content[pos..]){
            tokens.push(TokenTypes::TokAdd);
            pos += matched.len();
        }
        else if let Some(matched) = RE_MULT.find(&content[pos..]){
            tokens.push(TokenTypes::TokMult);
            pos += matched.len();
        }
        else if let Some(matched) = RE_DIV.find(&content[pos..]){
            tokens.push(TokenTypes::TokDiv);
            pos += matched.len();
        }
        else if let Some(matched) = RE_PINT.find(&content[pos..]){
            let value = match matched.as_str().parse::<i64>() {
                Ok(parsed_val) => {
                    parsed_val
                } 
                Err(err) => {
                    eprintln!("Error: {}", err);
                    0
                }
            };
            tokens.push(TokenTypes::TokInt(value));
            pos += matched.len();
        }
        else if let Some(matched) = RE_NOT.find(&content[pos..]){
            tokens.push(TokenTypes::TokNot);
            pos += matched.len();
        }

        else if let Some(matched) = RE_IF.find(&content[pos..]){
            tokens.push(TokenTypes::TokIf);
            pos += matched.len();
        }
        else if let Some(matched) = RE_ELSE.find(&content[pos..]){
            tokens.push(TokenTypes::TokElse);
            pos += matched.len();
        }
        else if let Some(matched) = RE_LET.find(&content[pos..]){
            tokens.push(TokenTypes::TokLet);
            pos += matched.len();
        }
        else if let Some(matched) = RE_REC.find(&content[pos..]){
            tokens.push(TokenTypes::TokRec);
            pos += matched.len();
        }
        else if let Some(matched) = RE_DEF.find(&content[pos..]){
            tokens.push(TokenTypes::TokDef);
            pos += matched.len();
        }
        else if let Some(matched) = RE_FUN.find(&content[pos..]){
            tokens.push(TokenTypes::TokFun);
            pos += matched.len();
        }
        else if let Some(matched) = RE_IN.find(&content[pos..]){
            tokens.push(TokenTypes::TokIn);
            pos += matched.len();
        }
        else if let Some(matched) = RE_THEN.find(&content[pos..]){
            tokens.push(TokenTypes::TokThen);
            pos += matched.len();
        }
        else if let Some(matched) = RE_BOOL.find(&content[pos..]){
            tokens.push(TokenTypes::TokNot);
            pos += matched.len();
        }
        else if let Some(matched) = RE_ARROW.find(&content[pos..]){
            tokens.push(TokenTypes::TokArrow);
            pos += matched.len();
        }
        else if let Some(matched) = RE_LPAREN.find(&content[pos..]){
            tokens.push(TokenTypes::TokLParen);
            pos += matched.len();
        }
        else if let Some(matched) = RE_RPAREN.find(&content[pos..]){
            tokens.push(TokenTypes::TokRParen);
            pos += matched.len();
        }
        else if let Some(matched) = RE_CONCAT.find(&content[pos..]){
            tokens.push(TokenTypes::TokConcat);
            pos += matched.len();
        }
        else if let Some(matched) = RE_DOUBLESEMI.find(&content[pos..]){
            tokens.push(TokenTypes::TokDoubleSemi);
            pos += matched.len();
        }
        else if let Some(matched) = RE_SUB.find(&content[pos..]){
            tokens.push(TokenTypes::TokSub);
            pos += matched.len();
        }
        else if let Some(matched) = RE_STRING.find(&content[pos..]){
            tokens.push(TokenTypes::TokString(matched.as_str().to_string()));
            pos += matched.len();
        }
        else if let Some(matched) = RE_ID.find(&content[pos..]){
            tokens.push(TokenTypes::TokID(matched.as_str().to_string()));
            pos += matched.len();
        }
        else{
            eprintln!("Unknown token in {}", &content[pos..]);
            break;
            
        }
        //     println!("matched position {}", matched.start());
        // } else if let Some(matched)=RE_{
        //     
        // }
    }

    tokens
}

fn main() {
     // dbg!(args);
    let file_path = match env::args().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("Usage: mycaml <file_path>");
            return;
        }
    };

    let content = fs::read_to_string(&file_path).expect(&format!("Error reading the file: {}", &file_path));
    let tokens = tokenize(&content);
    println!("Tokens: {:?}", tokens);
}


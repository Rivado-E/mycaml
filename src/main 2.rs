#[warn(unused_variables)]
#[warn(dead_code)]

use std::env;
use std::fs;
use regex::Regex;

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

static RE_WHITESPACE:Option<Regex> = Regex::new(r"^[\n\t\r ]+").unwrap();
static RE_STRING:Regex = Regex::new(r#".+\"#).unwrap();
static RE_LPAREN:Regex  = Regex::new(r"^\(").unwrap();
static RE_RPAREN:Regex = Regex::new(r"^\)").unwrap();
static RE_EQUAL:Regex = Regex::new(r"^=").unwrap();
static RE_NOTEQUAL:Regex = Regex::new(r"^<>").unwrap();
static RE_GREATER:Regex = Regex::new(r"^>").unwrap();
static RE_LESSER:Regex= Regex::new(r"^<").unwrap();
const RE_LESSEQUAL:Regex = Regex::new(r"^<=").unwrap();
const RE_GREATEREQUAL:Regex = Regex::new(r"^>=").unwrap();
const RE_OR:Regex = Regex::new(r"^||").unwrap();
const RE_AND:Regex = Regex::new(r"^&&").unwrap();
const RE_NOT:Regex = Regex::new(r"^not").unwrap();
const RE_IF:Regex = Regex::new(r"^if").unwrap();
const RE_THEN:Regex = Regex::new(r"^then").unwrap();
const RE_ELSE:Regex = Regex::new(r"^else").unwrap();
const RE_ADD:Regex = Regex::new(r"^\+").unwrap();
const RE_SUB:Regex = Regex::new(r"^-").unwrap();
const RE_MULT:Regex = Regex::new(r"^\*").unwrap();
const RE_DIV:Regex = Regex::new(r"^\/").unwrap();
const RE_CONCAT:Regex = Regex::new(r"^\^").unwrap();
const RE_LET:Regex = Regex::new(r"^let").unwrap();
const RE_REC:Regex = Regex::new(r"^rec").unwrap();
const RE_IN:Regex = Regex::new(r"^in").unwrap();
const RE_DEF:Regex = Regex::new(r"^def").unwrap();
const RE_FUN:Regex = Regex::new(r"^fun").unwrap();
const RE_ARROW:Regex = Regex::new(r"^->").unwrap();
const RE_PINT:Regex = Regex::new(r"^[0-9]+").unwrap();
const RE_NINT:Regex = Regex::new(r"^-[0-9]+").unwrap();
const RE_BOOL:Regex = Regex::new(r"^(false|true)").unwrap();
const RE_ID:Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*").unwrap();
const RE_DOUBLESEMI:Regex = Regex::new(r";;").unwrap();

fn tokenizer(content:String)-> Vec<TokenTypes>{
    
    let tokens :Vec<TokenTypes> = Vec::new();
    let file_size = content.len(); //may be needs to be a u64
    let position = 0;

    while position < file_size {
        if let Some(matched) = RE_WHITESPACE.find(&content[position..]){
            println!("matched position {}", matched.start());
        }
    }

    tokens
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let re = Regex::new(r"let[a-zA-Z]*").unwrap();
     // dbg!(args);
    let filename = env::args().nth(1).expect("no filename given");

    if args.len() > 1{

        let contents = fs::read_to_string(&filename)
            .expect(&filename);

        println!("Source code:\n{contents}");

        print!("the length of the content {}", contents.len());
    }   
}


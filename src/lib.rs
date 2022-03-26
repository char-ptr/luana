use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;


use full_moon::ast::punctuated::{Punctuated, Pair};
use full_moon::ast::{Ast, FunctionCall, Call, Do, Block, Stmt, Value, TableConstructor, Field, Expression};
use full_moon::node::Node;
use full_moon::tokenizer::{Token, TokenType, TokenReference};
use full_moon::visitors::*;
#[derive(Default)]
pub struct vistAst {
    project_dir: PathBuf,
    imports: Vec<String>,
    previous_imports: Vec<String>,
    previous_file_name: String,
    file_name: String,
}
impl VisitorMut for vistAst {
    fn visit_stmt(&mut self, node:full_moon::ast::Stmt) ->full_moon::ast::Stmt {
        let mut new_node = node.clone();
        match node {

            full_moon::ast::Stmt::FunctionCall(ref fnc) => {

                let prefix = fnc.prefix().to_string();

                match prefix.trim() {
                    "_load" => {
                        let call_paren = fnc.suffixes().next().expect("called load without filename");
                        match call_paren {
                            full_moon::ast::Suffix::Call(c) => {
                                match c {
                                    Call::AnonymousCall(ac) => {
                                        let mut load_file_name = String::new();
                                        match ac {
                                            full_moon::ast::FunctionArgs::Parentheses { parentheses: _, arguments } => {
                                                let first_arg = arguments.iter().next();
                                                if let Some(arg) = first_arg {
                                                    match arg {
                                                        full_moon::ast::Expression::Value { value, type_assertion } => {
                                                            match *value.clone() {
                                                                full_moon::ast::Value::String(s) => {
                                                                    load_file_name = self.get_token_name(s.token_type());
                                                                },
                                                                _ => {
                                                                    println!("please just pass a string...");
                                                                }
                                                            }
                                                        },
                                                        _ => {println!("im sorry but i really cannot be bothered parsing, please just pass a string ");}
                                                    }
                                                }
                                            },
                                            full_moon::ast::FunctionArgs::String(_) => todo!(),
                                            _ => {
                                                println!("invalid load @ {:?}", fnc.start_position());
                                            },
                                        }
                                        if !load_file_name.is_empty() {

                                            if load_file_name == self.previous_file_name {
                                                println!("detected circular import giving empty");
                                                let do_block = Do::new();
                                                // let do_block= do_block.with_block(ast.nodes().to_owned());
                                                new_node = Stmt::Do(do_block);
                                            
                                            }
                                            else {
                                                
                                                let load_path = self.project_dir.join(&load_file_name);
                                                let mut load_file = File::open(&load_path);
                                                if load_file.is_err() {
                                                    println!("Unable to open / find file @ {}", load_path.to_str().unwrap());
                                                }
                                                self.imports.push(load_file_name.clone());
                                                let mut code = String::new();
                                                let load_path_parent = load_path.parent().unwrap();
                                                load_file.unwrap().read_to_string(&mut code);
    
                                                let mut astvist = vistAst::default();
                                                astvist.set_project_dir(load_path_parent.to_path_buf());
                                                astvist.set_file_name(&load_file_name);
                                                astvist.previous_imports = self.imports.clone();
                                                astvist.previous_file_name = self.file_name.clone();
                                                let ast = full_moon::parse(&code);
                                                if let Ok(ast) = ast {
                                                    let ast = astvist.visit_ast(ast);
                                                    // let bblock = Block::new();
                                                    // bblock.with_stmts(ast.)
                                                    let do_block = Do::new();
                                                    let do_block= do_block.with_block(ast.nodes().to_owned());
                                                    new_node = Stmt::Do(do_block);
                                                    // println!("!o {:?}", &do_block);
                                                } else {
                                                    println!("Unable to parse file @ {}", load_path.to_str().unwrap());
                                                }
                                            }

                                        }
                                        println!("load file: {} | {:?}", load_file_name, self.project_dir);
                                    },
                                    _ => {},
                                }
                            },
                            _ => {},
                        }
                        // println!("{}", filename.to_string());
                    },
                    _=>{

                    }
                }
                // println!("{:?}", fnc.prefix().to_string());
            },
            _ => {}
        };
        new_node
    }
    fn visit_whitespace(&mut self, token:Token) ->Token {
     
        let mut new_token = token.clone();
        match token.token_type() {
            TokenType::Whitespace { characters } =>{
                // println!("ch - {:?}", characters);
                if characters.contains("\n") {
                    new_token = Token::new(TokenType::Symbol { symbol: full_moon::tokenizer::Symbol::Semicolon })
                }
            }
            _=>{},
        }

        new_token

    }
    fn visit_value(&mut self, node:full_moon::ast::Value) ->full_moon::ast::Value {
        let mut new_node = node.clone();
        match node {
            full_moon::ast::Value::String(s) => {
                let content = self.get_token_name(s.token_type());
                if content.starts_with("!import ") {
                    let file_name = &content[8..];
                    println!("importing file: {}", file_name);
                    let load_path = self.project_dir.join(file_name);
                    let mut load_file = File::open(&load_path);
                    if load_file.is_err() {
                        println!("Unable to open / find file @ {}", load_path.to_str().unwrap());
                    }
                    self.imports.push(file_name.to_string());
                    let mut file_content = String::new();
                    load_file.unwrap().read_to_string(&mut file_content);
                    match load_path.extension().unwrap().to_str().unwrap() {
                        "json" => {
                            let json_file: Result<serde_json::Value,_> = serde_json::from_str(&file_content);
                            if let Ok(json_file) = json_file {
                                // let table = self.json_value_to_lua(json_file);
                                new_node = self.json_value_to_lua(json_file)
                            }
                        },
                        _=>{
                            let token = Token::new(TokenType::StringLiteral { literal: file_content.replace("\r\n","\\n").replace("\t","\\t").into(), multi_line: None, quote_type: full_moon::tokenizer::StringLiteralQuoteType::Single });
                            let tkn_ref = s.with_token(token);
                            // tkn_ref.
                            new_node = Value::String(tkn_ref);
                        }
                    }
                }
                // println!("{}", content);
            },
            _ => {},
        }

        new_node
    }
}

impl vistAst {
    pub fn set_project_dir(&mut self, project_dir: PathBuf) {
        self.project_dir = project_dir;
    }
    pub fn set_file_name(&mut self, file_name: &str) {
        self.file_name = file_name.to_string();
    }
    pub fn set_previous_imports(&mut self, previous_imports: &Vec<String>) {
        self.previous_imports = previous_imports.clone();
    }
    fn get_token_name(&self, token: &TokenType) -> String {
        match token {
            TokenType::Eof => String::from("Eof"),
            TokenType::Identifier { identifier } => identifier.to_string(),
            // TokenType::MultiLineComment { blocks, comment } => todo!(),
            TokenType::Number { text } => text.to_string(),
            // TokenType::Shebang { line } => todo!(),
            // TokenType::SingleLineComment { comment } => todo!(),
            TokenType::StringLiteral { literal, multi_line, quote_type } => literal.to_string(),
            TokenType::Symbol { symbol } => symbol.to_string(),
            // TokenType::Whitespace { characters } => todo!(),
            _ => String::from("Unknown"),
        }
    }
    fn json_value_to_lua(&mut self,json:serde_json::Value)->Value {
        match json {
            serde_json::Value::Null => {
                Value::Symbol(TokenReference::new(vec![], Token::new(TokenType::Symbol { symbol: full_moon::tokenizer::Symbol::Nil }), vec![]))
            },
            serde_json::Value::Bool(b) => {
                let sym = if b {
                    Token::new(TokenType::Symbol { symbol: full_moon::tokenizer::Symbol::True })
                } else {
                    Token::new(TokenType::Symbol { symbol: full_moon::tokenizer::Symbol::False })
                };
                Value::Symbol(TokenReference::new(vec![], sym, vec![]))
            },
            serde_json::Value::Number(n) => {
                Value::Number(TokenReference::new(vec![], Token::new(TokenType::Number { text: n.to_string().into() }), vec![]))
            },
            serde_json::Value::String(s) => {
                Value::String(TokenReference::new(vec![Token::new(TokenType::Symbol { symbol: full_moon::tokenizer::Symbol::LeftBracket })], 
                    Token::new(TokenType::StringLiteral  { literal: s.to_string().into(), multi_line:None,quote_type: full_moon::tokenizer::StringLiteralQuoteType::Single }), 
                    vec![Token::new(TokenType::Symbol { symbol: full_moon::tokenizer::Symbol::RightBracket })]))
            },
            serde_json::Value::Array(a) => {
                let t = TableConstructor::new();
                let mut punct = Punctuated::<Field>::new(); 
                for val in a {
                    let expr = Expression::Value { value: Box::from(self.json_value_to_lua(val)), type_assertion: None };
                    let field = Field::NoKey(expr);
                    punct.push(Pair::new(field, Some(TokenReference::symbol(",").unwrap())));
                }
                Value::TableConstructor(t.with_fields(punct))
            },
            serde_json::Value::Object(o) => {
                let t = TableConstructor::new();
                let mut punct = Punctuated::<Field>::new(); 
                for (k,val) in o {
                    let expr = Expression::Value { value: Box::from(self.json_value_to_lua(val)), type_assertion: None };
                    let field = Field::NameKey{
                        key: TokenReference::new(vec![Token::new(TokenType::Symbol { symbol: full_moon::tokenizer::Symbol::LeftBracket })], 
                        Token::new(TokenType::StringLiteral  { literal: k.to_string().into(), multi_line:None,quote_type: full_moon::tokenizer::StringLiteralQuoteType::Single }), 
                        vec![Token::new(TokenType::Symbol { symbol: full_moon::tokenizer::Symbol::RightBracket })]),
                        equal: TokenReference::symbol("=").unwrap(),
                        value:expr
                    };
                    punct.push(Pair::new(field, Some(TokenReference::symbol(",").unwrap())));
                }
                Value::TableConstructor(t.with_fields(punct))
            },
        }
    }
}

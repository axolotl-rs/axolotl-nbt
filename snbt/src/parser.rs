use std::collections::HashMap;
use logos::Lexer;
use serde::de::Unexpected::Option;
use crate::{Error, NameLessValue, Token, Value};

pub fn parse(mut lex: Lexer<Token>) -> Result<Value, Error> {
    if let Some(token) = lex.next() {
        match token {
            Token::StartCompound => {
                let value = parse_compound(&mut lex)?;
                Ok(Value::Compound {
                    name: "".to_string(),
                    value,
                })
            }
            _ => {
                Err(Error::UnexpectedToken(token.clone()))
            }
        }
    } else {
        Ok(Value::End)
    }
}

pub fn parse_compound(lex: &mut Lexer<Token>) -> Result<Vec<Value>, Error> {
    let mut last_name = None;
    let mut values = Vec::new();
    while let Some(token) = lex.next() {
        match token {
            Token::ByteArray => {
                let value = parse_byte_array(lex)?;
                values.push(Value::ByteArray { name: last_name.ok_or(Error::MissingName)?, value });
                last_name = None;
            }
            Token::IntArray => {
                let value = parse_int_array(lex)?;
                values.push(Value::IntArray { name: last_name.ok_or(Error::MissingName)?, value });
                last_name = None;
            }
            Token::LongArray => {
                let value = parse_long_array(lex)?;
                values.push(Value::LongArray { name: last_name.ok_or(Error::MissingName)?, value });
                last_name = None;
            }
            Token::StartList => {
                let value = parse_list(lex)?;
                values.push(Value::List { name: last_name.ok_or(Error::MissingName)?, value });
                last_name = None;
            }
            Token::EndListOrArray => {}
            Token::StartCompound => {
                let value = parse_compound(lex)?;
                values.push(Value::Compound {
                    name: last_name.ok_or(Error::MissingName)?,
                    value,
                });
                last_name = None;
            }
            Token::EndCompound => {
                break;
            }
            Token::Byte(b) => {
                values.push(Value::Byte {
                    name: last_name.ok_or(Error::MissingName)?,
                    value: b,
                });
                last_name = None;
            }
            Token::Short(s) => {
                values.push(Value::Short {
                    name: last_name.ok_or(Error::MissingName)?,
                    value: s,
                });
                last_name = None;
            }
            Token::Int(i) => {
                values.push(Value::Int {
                    name: last_name.ok_or(Error::MissingName)?,
                    value: i,
                });
                last_name = None;
            }
            Token::Long(l) => {
                values.push(Value::Long {
                    name: last_name.ok_or(Error::MissingName)?,
                    value: l,
                });
                last_name = None;
            }
            Token::Float(v) => {
                values.push(Value::Float {
                    name: last_name.ok_or(Error::MissingName)?,
                    value: v,
                });
                last_name = None;
            }
            Token::True => {
                values.push(Value::Boolean {
                    name: last_name.ok_or(Error::MissingName)?,
                    value: true,
                });
                last_name = None;
            }
            Token::False => {
                values.push(Value::Boolean {
                    name: last_name.ok_or(Error::MissingName)?,
                    value: false,
                });
                last_name = None;
            }
            Token::TagName(name) => {
                last_name = Some(name);
            }
            Token::String(string) => {
                values.push(Value::String {
                    name: last_name.ok_or(Error::MissingName)?,
                    value: string,
                });
                last_name = None;
            }
            Token::ArrayListCompoundSeparator => {}
            Token::Error => {
                return Err(Error::UnexpectedToken(token.clone()));
            }
        }
    }
    Ok(values)
}
fn parse_list(lex: &mut Lexer<Token>) -> Result<Vec<NameLessValue>, Error>{
    let mut values = Vec::new();
    while let Some(token) = lex.next() {
        match token {
            Token::StartList => {
                let value = parse_list(lex)?;
                values.push(NameLessValue::List(value));
            }
            Token::EndListOrArray => {
                break;
            }
            Token::StartCompound => {
                let value = parse_compound(lex)?;
                values.push(NameLessValue::Compound(value));
            }
            Token::Byte(b) => {
                values.push(NameLessValue::Byte(b));
            }
            Token::Short(s) => {
                values.push(NameLessValue::Short(s));
            }
            Token::Int(i) => {
                values.push(NameLessValue::Int(i));
            }
            Token::Long(l) => {
                values.push(NameLessValue::Long(l));
            }
            Token::Float(v) => {
                values.push(NameLessValue::Float(v));
            }
            Token::True => {
                values.push(NameLessValue::Bool(true));
            }
            Token::False => {
                values.push(NameLessValue::Bool(false));
            }
            Token::String(string) => {
                values.push(NameLessValue::String(string));
            }
            Token::ArrayListCompoundSeparator => {}
            Token::Error => {
                return Err(Error::UnexpectedToken(token.clone()));
            }
            _ => {}
        }
    }
    Ok(values)

}
fn parse_byte_array(lex: &mut Lexer<Token>) -> Result<Vec<i8>, Error> {
    let mut values = Vec::new();
    while let Some(token) = lex.next() {
        match token {
            Token::Byte(b) => {
                values.push(b);
            }
            Token::EndListOrArray => {
                break;
            }
            Token::Error => {
                return Err(Error::UnexpectedToken(token.clone()));
            }
            _ => {
                return Err(Error::UnexpectedToken(token.clone()));
            }
        }
    }
    Ok(values)
}

fn parse_int_array(lex: &mut Lexer<Token>) -> Result<Vec<i32>, Error> {
    let mut values = Vec::new();
    while let Some(token) = lex.next() {
        match token {
            Token::Int(i) => {
                values.push(i);
            }
            Token::EndListOrArray => {
                break;
            }
            Token::Error => {
                return Err(Error::UnexpectedToken(token.clone()));
            }
            _ => {
                return Err(Error::UnexpectedToken(token.clone()));
            }
        }
    }
    Ok(values)
}

fn parse_long_array(lex: &mut Lexer<Token>) -> Result<Vec<i64>, Error> {
    let mut values = Vec::new();
    while let Some(token) = lex.next() {
        match token {
            Token::Long(l) => {
                values.push(l);
            }
            Token::EndListOrArray => {
                break;
            }
            Token::Error => {
                return Err(Error::UnexpectedToken(token.clone()));
            }
            _ => {
                return Err(Error::UnexpectedToken(token.clone()));
            }
        }
    }
    Ok(values)
}
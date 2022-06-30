mod error;
pub(crate) mod lexer;
mod parser;
mod value;

use crate::lexer::Token;
pub use error::Error;
use logos::Logos;
pub use value::{NameLessValue, Value};

#[cfg(test)]
mod tests {

    use crate::to_value;

    #[test]
    fn it_works() {
        let value =
            to_value(r#"{name1:123,name2:"sometext1",name3:{subname1:456,subname2:"sometext2"}}"#)
                .unwrap();
        println!("{:?}", value);
    }
}

pub fn to_value(str: &str) -> Result<Value, Error> {
    let lex = Token::lexer(str);
    let value = parser::parse(lex)?;
    Ok(value)
}

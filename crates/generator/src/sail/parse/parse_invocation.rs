use super::parse_list;
use crate::model::FunctionInvocation;
use crate::model::Value;
use crate::sail::Parser;
use crate::sail::Token;

pub fn parse_invocation(p: &mut Parser) -> crate::Result<Value> {
    let name = p.identifier()?;
    p.expect(Token::OpenParen)?;
    let args = parse_list(p)?;

    if args.len() == 1 && matches!(args[0], Value::Unit) {
        Ok(Value::FunctionInvocation(FunctionInvocation {
            name,
            args: Vec::new(),
        }))
    } else {
        Ok(Value::FunctionInvocation(FunctionInvocation { name, args }))
    }
}

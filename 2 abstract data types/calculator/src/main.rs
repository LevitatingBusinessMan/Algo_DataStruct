use std::iter::Peekable;
use std::io::Write;

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    loop {
        print!("calc> ");
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut tree = parse_expression(&mut input.chars().filter(|c| !c.is_whitespace()).peekable());
        println!("{}", tree.interpret());
    }

}

#[derive(Debug)]
enum Value {
    BinaryExpression(Box<BinaryExpression>),
    Int(u32),
}

#[derive(Debug)]
enum BinaryExpression {
    Addition((Value, Value)),
    Subtraction((Value, Value)),
    Multiplication((Value, Value)),
    Division((Value, Value)),
}

trait Interpret {
    fn interpret(&mut self) -> u32;
}

impl Interpret for Value {
    fn interpret(&mut self) -> u32 {
        match self {
            Value::Int(int) => *int,
            Value::BinaryExpression(expr) => expr.interpret()
        }
    }
}

impl Interpret for BinaryExpression {
    fn interpret(&mut self) -> u32 {
        match self {
            BinaryExpression::Addition(tup) => tup.0.interpret() + tup.1.interpret(),
            BinaryExpression::Subtraction(tup) => tup.0.interpret() - tup.1.interpret(),
            BinaryExpression::Multiplication(tup) => tup.0.interpret() * tup.1.interpret(),
            BinaryExpression::Division(tup) => tup.0.interpret() / tup.1.interpret(),
        }
    }
}

fn consume<I: Iterator<Item=char>>(source: &mut Peekable<I>, expectation: char) {
    let token = source.next().unwrap();
    if token != expectation {
        panic!("Unexpected token {}", token);
    }
}


fn parse_expression<I: Iterator<Item=char>>(source: &mut Peekable<I>) -> Value {    
    consume(source, '(');
    let left = match source.peek().unwrap() {
        '(' => parse_expression(source),
        _ => Value::Int(source.next().unwrap().to_digit(10).expect("Expected an integer value")),
    };

    let operator = source.next().unwrap();

    let right = match source.peek().unwrap() {
        '(' => parse_expression(source),
        _ => Value::Int(source.next().unwrap().to_digit(10).expect("Expected an integer value")),
    };

    consume(source, ')');

    Value::BinaryExpression(Box::new(match operator {
        '+' => BinaryExpression::Addition((left, right)),
        '-' => BinaryExpression::Subtraction((left, right)),
        '*' => BinaryExpression::Multiplication((left, right)),
        '/' => BinaryExpression::Division((left, right)),
        _ => panic!("Ye that's not an operator")
    }))
}

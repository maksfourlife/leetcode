// https://leetcode.com/problems/evaluate-reverse-polish-notation/

fn eval_rpn<I>(tokens: I) -> i32
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut stack = vec![];

    for token in tokens.into_iter() {
        let op = match token.as_ref() {
            "+" => Operand::Add,
            "-" => Operand::Sub,
            "*" => Operand::Mul,
            "/" => Operand::Div,
            s => Operand::Val(s.parse().unwrap()),
        };
        match op {
            Operand::Add => {
                let b = stack.pop().unwrap();
                let a = stack.last_mut().unwrap();
                *a += b;
            }
            Operand::Sub => {
                let b = stack.pop().unwrap();
                let a = stack.last_mut().unwrap();
                *a -= b;
            }
            Operand::Mul => {
                let b = stack.pop().unwrap();
                let a = stack.last_mut().unwrap();
                *a *= b;
            }
            Operand::Div => {
                let b = stack.pop().unwrap();
                let a = stack.last_mut().unwrap();
                *a /= b;
            }
            Operand::Val(val) => stack.push(val),
        }
    }

    stack[0]
}

enum Operand {
    Add,
    Sub,
    Mul,
    Div,
    Val(i32),
}

fn main() {
    assert_eq!(eval_rpn(&["2", "1", "+", "3", "*"]), 9);
    assert_eq!(eval_rpn(&["4", "13", "5", "/", "+"]), 6);
    assert_eq!(
        eval_rpn(&[
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
        ]),
        22
    );
}

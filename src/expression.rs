use std::ops::Add;
use std::vec::Vec;

pub enum Cardinality {
    _1,
    _2,
    _N,
}

pub trait FunctionDef {
    fn name(&self) -> String;
    fn cardinality(&self) -> Cardinality;
    fn inner_complexity(&self) -> u16;
}

pub enum Functions {
    Add,
    Mult,
    Cos,
}

impl FunctionDef for Functions {
    fn name(&self) -> String {
        match self {
            Functions::Add => String::from("+"),
            Functions::Mult => String::from("*"),
            Functions::Cos => String::from("cos"),
        }
    }
    fn cardinality(&self) -> Cardinality {
        match self {
            Functions::Add => Cardinality::_N,
            Functions::Mult => Cardinality::_N,
            Functions::Cos => Cardinality::_1,
        }
    }
    fn inner_complexity(&self) -> u16 {
        match self {
            Functions::Add => 1,
            Functions::Mult => 5,
            Functions::Cos => 30,
        }
    }
}

pub enum Expression {
    Constant(i64),
    Expr(Functions, Vec<Expression>),
}
pub trait Complexity {
    fn calc(&self) -> u16 {
        0
    }
}
impl Complexity for Expression {
    fn calc(&self) -> u16 {
        match self {
            Expression::Constant(x) => 0,
            Expression::Expr(f, args) => {
                let args_comp: u16 = args.iter().map(Complexity::calc).sum();
                f.inner_complexity().add(args_comp).add(args.len() as u16)
            }
        }
    }
}

pub trait Simplify {
    fn simply(_: Self) -> Self;
}

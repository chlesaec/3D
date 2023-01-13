use std::cmp::Ordering;
use std::ops::{Add, Mul};
use std::marker::{Copy, PhantomData};
use crate::expression::Expression::Expr;


pub trait Group : Add<Self> + Copy + Clone {
    fn zero() -> Self;

    fn minus(&self) -> Self;
}

pub trait Ring : Group + Mul<Self> {
    fn neutral() -> Self;

    fn inverse(&self) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct Vector<T : Ring, const N: usize> {
    pub values: [T; N],
}

pub trait MatrixElement<T : Ring> : Ring {
    fn mult(&self, x : T) -> T;

    fn multiply(&self, r: Self) -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix<T, U, const N: usize> where U : MatrixElement<T>, T : Ring {
    pub elements : [Vector<U, N>; N],
    _t: PhantomData<T>,
}

impl<T: Ring + Add<Output = T>, const N: usize> Add for Vector<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result :  [T; N] = [T::zero(); N];
        for index in 0..N {
            result[index] = rhs.values[index] + self.values[index];
        }
        let p : Vector<T,N> = Vector {
            values: result,
        };
        return p;
    }
}

impl <T: Ring + Add<Output = T> + Mul<Output = T>, const N: usize> Mul for Vector<T, N> {
    type Output = T;

    fn mul(self, rhs: Vector<T, N>) -> Self::Output {
        let mut result : T = T::zero();
        for index in 0..N {
            result = result + (self.values[index] * rhs.values[index]);
        }
        return result;
    }
}

impl<T: Ring + Add<Output = T>, const N: usize> Group for Vector<T, N> {
    fn zero() -> Self {
        let result :  [T; N] = [T::zero(); N];
        let p : Vector<T,N> = Vector {
            values: result,
        };
        return p;
    }

    fn minus(&self) -> Self {
        let mut p: Vector<T, N> = Self::zero();

        for i  in 0..p.values.len() {
            p.values[i] = self.values[i].minus();
        }
        return p;
    }
}

impl<T : Ring + Add<Output = T> + Mul<Output = T>, const N: usize> Ring for Vector<T,N> {
    fn neutral() -> Self {
        let result : Vector<T, N> = Vector {
            values: [T::neutral(); N]
        };
        return result;
    }

    fn inverse(&self) -> Self {
        let mut p: Vector<T, N> = Self::neutral();

        for i  in 0..p.values.len() {
            p.values[i] = self.values[i].inverse();
        }
        return p;
    }
}


impl<T: Ring + Add<Output = T> + Mul<Output = T>, U: MatrixElement<T> + Add<Output = U> + Mul<Output = U>, const N: usize> Add for Matrix<T, U, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result : [Vector<U, N>; N] = self.elements.clone();
        for index in 0..N {
            result[index] = result[index] + rhs.elements[index];
        }
        return Matrix {
            elements: result,
            _t: Default::default()
        }
    }
}

impl<T: Ring + Add<Output = T> + Mul<Output = T>, U: MatrixElement<T> + Add<Output = U> + Mul<Output = U>, const N: usize> Group for Matrix<T, U, N> {
    fn zero() -> Self {
        let result : [Vector<U, N>; N] = [Vector {
            values: [U::zero(); N]
        }; N];
        return Matrix {
            elements: result,
            _t: Default::default()
        }
    }

    fn minus(&self) -> Self {
        return *self; // todo!()
    }
}

impl<T: Ring + Add<Output = T> + Mul<Output = T>, U: MatrixElement<T> + Add<Output = U> + Mul<Output = U>, const N: usize> Mul for Matrix<T, U, N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result : Matrix<T, U, N> = Matrix::zero();
        for line in 0..N {
            for col in 0.. N {
                for index in 0..N {
                    result.elements[col].values[line] = result.elements[col].values[line]
                        + self.elements[line].values[index]*rhs.elements[index].values[col];
                }
            }
        }
        return result;
    }
}

impl<T: Ring + Add<Output = T> + Mul<Output = T>, U: MatrixElement<T> + Add<Output = U> + Mul<Output = U>, const N: usize> Ring for Matrix<T, U, N> {
    fn neutral() -> Self {
        let mut result : [Vector<U, N>; N] = [Vector {
            values: [U::zero(); N]
        }; N];
        for line in 0..N {
            result[line].values[line] = U::neutral()
        }
        return Matrix {
            elements: result,
            _t: Default::default()
        }
    }

    fn inverse(&self) -> Self {
        return *self; // todo!()
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, PartialOrd, Ord)]
pub enum Function {
    Sum,
    Sub,
    Product,
    Div,
    Cos,
    Sin
}

impl Function {
    pub fn calc(&self, args: Vec<f64>) -> f64 {
        match self {
            Function::Sum => { args.iter().sum() }
            Function::Sub => {
                if args.len() != 2 {
                    panic!("Substraction must have 2 args")
                }
                args[0] - args[1]
            }
            Function::Product => { args.iter().fold(1.0f64, |p, x| { p*x } )}
            Function::Div=> {
                if args.len() != 2 {
                    panic!("Division must have 2 args")
                }
                if args[1] == 0.0 {
                    panic!("Div by zero")
                }
                args[0] / args[1]
            }
            Function::Cos=> {
                if args.len() != 1 {
                    panic!("Cos must have 1 args")
                }
                args[0].cos()
            }
            Function::Sin=> {
                if args.len() != 1 {
                    panic!("Sin must have 1 args")
                }
                args[0].sin()
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Expression<T: Ring + Add<Output = T> + Mul<Output = T> + PartialEq + Eq> {
    Cons(T),
    Exp{ f: Function, arguments: Vec<Expression<T>> }
}

impl<T: Ring + Add<Output = T> + Mul<Output = T> + PartialEq + Eq + Ord> Ord for Expression<T>  {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Expression::Cons(x), Expression::Cons(y)) => x.cmp(y),
            (Expression::Cons(_), _) => Ordering::Less,
            (_, Expression::Cons(_)) => Ordering::Greater,
            (Expression::Exp {f:f1, arguments:args1},
                Expression::Exp {f:f2, arguments:args2})
                => {
                if f1.eq(f2) {
                    if args1.len() == args2.len() {
                        args1.cmp(args2)
                    }
                    else {
                        args1.len().cmp(&args2.len())
                    }
                }
                else {
                    f1.cmp(f2)
                }
            }
        }
    }
}

impl<T: Ring + Add<Output = T> + Mul<Output = T> + PartialEq + Eq + Ord> PartialOrd for Expression<T>  {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl<T: Ring + Add<Output = T> + Mul<Output = T> + PartialEq + Eq> Clone for Expression<T> {
    fn clone(&self) -> Self {
        match self {
            Expression::Cons(x) => Expression::Cons(x.clone()),
            Expression::Exp {f: f1, arguments: arguments1}
               => Expression::Exp {f : *f1, arguments: arguments1.clone() }
        }
    }
}

impl<T: Ring + Add<Output = T> + Mul<Output = T> + PartialEq + Eq + Ord> Expression<T> {
    pub fn simplify(&self) -> Expression<T> {
        return match self {
            Self::Cons(x) => Self::Cons(*x),
            Self::Exp{f, arguments} => Expression::simplify_call(*f, arguments)
        }
    }

    fn simplify_call(f_in: Function, arguments_in: &Vec<Expression<T>>) -> Expression<T> {
        return match f_in {
            Function::Sum => Self::simplify_add(arguments_in),
            Function::Sub => Self::simplify_sub(arguments_in),
            Function::Product => Self::simplify_mul(arguments_in),
            Function::Cos => Self::simplify_cos(arguments_in),
            _ => Self::Exp{ f : f_in, arguments: arguments_in.clone() }
        }
    }

    fn simplify_add(arguments: &Vec<Expression<T>>) -> Expression<T> {
        if arguments.len() == 1 {
            return arguments[0].simplify();
        }

        // Add all cons
        let mut args : Vec<Expression<T>> = arguments.to_vec();
        let mut index1 : usize = 0;
        let mut first : usize = 0;
        let mut start_ok: bool = false;
        let mut sum : T = T::zero();
        while index1 < args.len() {
            args[index1] = args[index1].simplify();
            let arg_indexed = args[index1].clone();
            match arg_indexed {
                Expression::Cons(v1) => {
                    sum = sum + v1;
                    if !start_ok {
                        start_ok = true;
                        first = index1;
                        index1 += 1;
                    }
                    else {
                        args.remove(index1);
                    }
                },
                Expression::Exp { f: Function::Sum, arguments:args_sum} => {
                    for a in args_sum {
                        args.push(a.clone())
                    }
                },
                _ => index1 += 1
            }
        }
        if start_ok {
            args[first] = Expression::Cons(sum);
        }
        if arguments.len() == 1 {
            return arguments[0].simplify();
        }
        args.sort();

        return Self::Exp{ f: Function::Sum, arguments: args };
    }

    fn simplify_mul(arguments: &Vec<Expression<T>>) -> Expression<T> {
        if arguments.len() == 1 {
            return arguments[0].simplify();
        }

        // Add all cons
        let mut args : Vec<Expression<T>> = arguments.to_vec();
        let mut index1 : usize = 0;
        let mut first : usize = 0;
        let mut start_ok: bool = false;
        let mut product: T = T::neutral();
        while index1 < args.len() {
            args[index1] = args[index1].simplify();
            let arg_indexed = args[index1].clone();
            match arg_indexed {
                Expression::Cons(v1) => {
                    product = product * v1;
                    if !start_ok {
                        start_ok = true;
                        first = index1;
                        index1 += 1;
                    }
                    else {
                        args.remove(index1);
                    }
                },
                Expression::Exp { f: Function::Product, arguments:args_product} => {
                    for a in args_product {
                        args.push(a.clone())
                    }
                },
                _ => index1 += 1
            }
        }
        if product == T::zero() {
            return Expression::Cons(T::zero());
        }
        if start_ok {
            args[first] = Expression::Cons(product);
        }
        if arguments.len() == 1 {
            return arguments[0].simplify();
        }
        args.sort();
        index1 = 0;
        while index1 + 1 < args.len() {
            let f1 = &args[index1];
            let f2 = &args[index1 + 1];
            match (f1, f2) {
                (Expression::Exp {f: Function::Cos, arguments: a1},
                Expression::Exp { f:Function::Cos, arguments: a2 }) => { // cos(a)*cos(b) = (cos(a+b) + cos(a-b))/2
                    let res = Expression::Exp {f:Function::Div,
                        arguments: vec![
                            Expression::Exp { f:Function::Sum, arguments:vec![
                                Expression::Exp { f:Function::Cos, arguments:vec![
                                    Expression::Exp { f:Function::Sum, arguments:vec![a1[0].clone(), a2[0].clone()] }.simplify()
                                ]},
                                Expression::Exp { f:Function::Cos, arguments:vec![
                                    Expression::Exp { f: Function::Sub, arguments:vec![a1[0].clone(), a2[0].clone()] }.simplify()
                                ]}
                            ]},
                            Expression::Exp{ f:Function::Sum, arguments:vec![Expression::Cons(T::neutral()),
                                                                             Expression::Cons(T::neutral())]}.simplify()
                        ]};
                    args[index1] = res;
                    args.remove(index1 + 1);
                },
                (_, _) => index1 += 1
            }
        }
        return Self::Exp{ f: Function::Product, arguments: args };
    }

    fn simplify_cos(args: &Vec<Expression<T>>) -> Expression<T> {
        let arg1: Option<&Expression<T>> = args.get(0);
        match arg1 {
            (Some(Expression::Cons(a))) => {
                if *a == T::zero() {
                    Expression::Cons(T::neutral())
                }
                else {
                    Expression::Exp { f: Function::Cos, arguments: args.clone() }
                }
            },
            _ => Expression::Exp {f: Function::Cos, arguments: args.clone() }
        }
    }

    fn simplify_sub(args: &Vec<Expression<T>>) -> Expression<T> {
        let arg1: Option<Expression<T>> = args.get(0).map(|x| { x.simplify() });
        let arg2: Option<Expression<T>> = args.get(1).map(|x| { x.simplify() });
        match (arg1, arg2) {
            (Some(Expression::Cons(a)), Some(Expression::Cons(b))) => Expression::Cons(a + b.minus()),
            (Some(e),  Some(Expression::Cons(b))) => Expression::Exp {f: Function::Sum,
                arguments : vec![e.clone(), Expression::Cons(b.minus())]},
            (Some(e1), Some(e2)) => {
                if e1.eq(&e2) {
                    Expression::Cons(T::zero())
                }
                else {
                    Expression::Exp {f: Function::Sub, arguments: args.clone() }
                }
            },
            (_, _) => Expression::Exp {f: Function::Sub, arguments: args.clone() }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Add;

    #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct StrTest {
        pub content: i32
    }


    impl Group for StrTest {
        fn zero() -> StrTest {
            return StrTest { content: 0 };
        }

        fn minus(&self) -> Self {
            return StrTest { content: -self.content };
        }
    }

    impl Add for StrTest {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                content: self.content + other.content
            }
        }
    }

    impl Ring for StrTest {
        fn neutral() -> Self {
            return StrTest { content: 1 };
        }

        fn inverse(&self) -> Self {
            StrTest { content : 1/self.content }
        }
    }

    impl Mul for StrTest {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            Self {
                content: self.content * other.content
            }
        }
    }

    #[test]
    fn add() {
        let x1 : Vector<StrTest, 2> = Vector {
            values: [
                StrTest { content: 5 },
                StrTest { content: 7 }
            ]
        };
        let x2 : Vector<StrTest, 2> = Vector {
            values: [
                StrTest { content: 3 },
                StrTest { content: 9 }
            ]
        };
        let x3: Vector<StrTest, 2> = x1 + x2;
        assert_eq!(8, x3.values[0].content);
        assert_eq!(16, x3.values[1].content);
    }

    #[test]
    fn simplify() {
        let x1: Expression<StrTest> = Expression::Cons(StrTest { content: 1});
        let x2: Expression<StrTest> = Expression::Exp {f: Function::Cos, arguments: vec![x1.clone()] };
        let x3: Expression<StrTest> = Expression::Cons(StrTest { content: 2});
        let x4: Expression<StrTest> = Expression::Cons(StrTest { content: 3});
        let exp : Expression<StrTest> = Expression::Exp {f: Function::Sum, arguments: vec![x1.clone(), x2.clone(), x3, x4] };
        let result: Expression<StrTest> = exp.simplify();

        let expected_result: Expression<StrTest> = Expression::Exp {f: Function::Sum,
            arguments: vec![Expression::Cons(StrTest { content: 6}), x2.clone()] };
        assert_eq!(expected_result, result);

        let mul_zero : Expression<StrTest> = Expression::Exp {f: Function::Product, arguments: vec![x2.clone(), x1.clone(), Expression::Cons(StrTest { content: 0})]};
        assert_eq!(Expression::Cons(StrTest { content: 0}), mul_zero.simplify());

        let minus_same : Expression<StrTest> = Expression::Exp {f: Function::Sub, arguments: vec![exp.clone(), expected_result.clone()]};
        assert_eq!(Expression::Cons(StrTest { content: 0}), minus_same.simplify());

        match result {
            Expression::Exp {f, arguments} => {
                assert_eq!(f, Function::Sum);
                assert_eq!(arguments.len(), 2);
                let mut nbc: u16 = 0;
                let mut nbf : u16 = 0;
                match arguments.get(0) {
                    Option::Some(Expression::Cons(x)) => {
                        assert_eq!(6, x.content);
                        nbc = nbc + 1;
                    }
                    Option::Some(Expression::Exp{f:f1, arguments:arguments1}) => {
                        assert_eq!(Function::Cos, *f1);
                        assert_eq!(1, arguments1.len());
                        nbf = nbf + 1;
                    }
                    _ => panic!("not good type")
                }
                match arguments.get(1) {
                    Option::Some(Expression::Cons(x)) => {
                        assert_eq!(6, x.content);
                        nbc = nbc + 1;
                    }
                    Option::Some(Expression::Exp{f:f1, arguments:arguments1}) => {
                        assert_eq!(Function::Cos, *f1);
                        assert_eq!(1, arguments1.len());
                        nbf = nbf + 1;
                    }
                    _ => panic!("not good type")
                }
                assert_eq!(1, nbc);
                assert_eq!(1, nbf);
            },
            _ => panic!("not ok")
        }
    }
}

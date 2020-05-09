// use crate::parser_mod::ParseItem::{Expression, Statement, Prefix, Infix};
// use crate::evaluation_mod::env::Env;

// #[derive(Clone, PartialEq)]
// pub enum Object {
//     Null,
//     Integer(i64),
//     String(String),
//     Boolean(bool),
//     Return(Box<Object>),
//     Function(String, Vec<String>, Vec<Statement>),
// }

// fn eval_expr(expression: Expression, env: &mut Env) -> Object {
//     match expression {
//         Expression::Integer(num) => Object::Integer(num),
//         Expression::Bool(_bool) => Object::Boolean(_bool),
//         Expression::Identifier(name) => env.get(&name.as_str()).expect(format!("{} not found", name).as_str()),
//         Expression::Function(ident, params, body) => Object::Function(ident, params, body),
//         Expression::Call(func, params) => {

//         }
//         Expression::Prefix(Prefix::Not, expression) => {
//             match eval_expr(*expression, env) {
//                 Object::Boolean(_bool) => Object::Boolean(!_bool),
//                 _ => {
//                     panic!("Can use ! operator only for booleans")
//                 }
//             }
//         },
//         Expression::Prefix(Prefix::Minus, expression) => {
//             match eval_expr(*expression, env) {
//                 Object::Integer(num) => Object::Integer(-num),
//                 _ => panic!("Can use the '-' operator only for numbers"),
//             }
//         },
//         Expression::Infix(Infix::Plus, lhs, rhs) => {
//             match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
//                 (Object::Integer(num1), Object::Integer(num2)) => Object::Integer(num1+num2),
//                 _ => panic!("Can only add integer literals"),
//             }
//         },
//         Expression::Infix(Infix::Minus, lhs, rhs) => {
//             match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
//                 (Object::Integer(num1), Object::Integer(num2)) => Object::Integer(num1 - num2),
//                 _ => panic!("Can only subtract integer literals"),
//             }
//         }
//         Expression::Infix(Infix::Divide, lhs, rhs) => {
//             match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
//                 (Object::Integer(num1), Object::Integer(num2)) => Object::Integer(num1 / num2),
//                 _ => panic!("Can only divide integer literals"),
//             }
//         }
//         Expression::Infix(Infix::Multiply, lhs, rhs) => {
//             match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
//                 (Object::Integer(num1), Object::Integer(num2)) => Object::Integer(num1 * num2),
//                 _ => panic!("Can only multiply integer literals"),
//             }
//         },
//         Expression::Infix(Infix::Equal, lhs, rhs) => {
//             match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
//                 (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 == num2),
//                 _ => panic!("Can only compare integer literals"),
//             }
//         },
//         Expression::Infix(Infix::NotEqual, lhs, rhs) => {
//             match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
//                 (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 != num2),
//                 _ => panic!("Can only compare integer literals")
//             }
//         },
//         Expression::Infix(Infix::MoreThanAndEqual, lhs, rhs) => {
//             match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
//                 (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 >= num2),
//                 _ => panic!("Can only compare integer literals")
//             }
//         },
//         Expression::Infix(Infix::MoreThan, lhs, rhs) => {
//             match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
//                 (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 > num2),
//                 _ => panic!("Can only compare integer literals")
//             }
//         },
//         Expression::Infix(Infix::LessThanAndEqual, lhs, rhs) => {
//             match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
//                 (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 <= num2),
//                 _ => panic!("Can only compare integer literals")
//             }
//         },
//         Expression::Infix(Infix::LessThan, lhs, rhs) => {
//             match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
//                 (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 < num2),
//                 _ => panic!("Can only compare integer literals")
//             }
//         },

//     }
// }
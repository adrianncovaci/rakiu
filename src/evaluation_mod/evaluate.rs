// use crate::parser_mod::ParseItem::{Expression, Statement, Prefix, Infix};
// use crate::evaluation_mod::env::Env;
// use crate::parser_mod::Parser::Parser;
// use crate::lexer_mod::lexer::Lexer;

// #[derive(Clone, PartialEq, Debug)]
// pub enum Object {
//     Null,
//     Integer(i64),
//     String(String),
//     Boolean(bool),
//     Return(Box<Object>),
//     Function(String, Vec<String>, Vec<Statement>),
//     Array(Vec<Object>),
// }

// fn eval_expr(expression: Expression, env: &mut Env) -> Object {
//     match expression {
//         Expression::Array(elems) => {
//             let els = elems.into_iter().map(|expr| eval_expr(expr., env)).collect();
//             Object::Array(els)
//         },
//         Expression::Integer(num) => Object::Integer(num),
//         Expression::Bool(_bool) => Object::Boolean(_bool),
//         Expression::Identifier(name) => env.get(&name.as_str()).expect(format!("{} not found", name).as_str()),
//         Expression::Function(ident, params, body) => Object::Function(ident, params, body),
//         Expression::Call{func, args} => {
//             let (params, body) = match *func {
//                 Expression::Identifier(name) => {
//                     match env.get(&name) {
//                         Some(Object::Function(_, args, body)) => (args, body),
//                         _ => {
//                             let elems = args.into_iter().map(|expr| eval_expr(expr, env)).collect();
//                             return eval_builtin(&name, elems).expect("Unrecognized function");
//                         }
//                     }
//                 },
//                 _ => panic!("yo, what's that?")
//                 };

//             if params.len() != args.len() {
//                 panic!("Wrong number of params");
//             }

//             let mut env2 = Env::new();
//             for(arg, val) in params.into_iter().zip(args.into_iter()) {
//                 env2.set(arg, eval_expr(val, env));
//             }

//             eval_return(body, &mut env2)

//         },
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
//         _ => Object::Null,
//     }
// }

// fn eval_builtin(name: &str, args: Vec<Object>) -> Option<Object> {
//     let mut env = Env::new();
//     match(name, args.as_slice()) {
//         ("size", [Object::Array(els)]) => Some(Object::Integer(els.len() as i64)),
//         ("max", [Object::Array(els)]) => {
//             let mut vec = vec![];
//             for el in els {
//                 match el {
//                     Object::Integer(num) => vec.push(*num),
//                     _ => continue,
//                 }
//             }
//             Some(Object::Integer(*vec.iter().max().unwrap()))
//         },
//         ("min", [Object::Array(els)]) => {
//             let mut vec = vec![];
//             for el in els {
//                 match el {
//                     Object::Integer(num) => vec.push(*num),
//                     _ => continue,
//                 }
//             }
//             Some(Object::Integer(*vec.iter().min().unwrap()))
//         },
//         ("sum", [Object::Array(els)]) => {
//             let mut vec = vec![];
//             for el in els {
//                 match el {
//                     Object::Integer(num) => vec.push(*num),
//                     _ => continue,
//                 }
//             }
//             let sum = vec.iter().sum::<i64>();
//             Some(Object::Integer(sum))
//         },
//         ("constant_product", [Object::Array(els), Object::Integer(_const)]) => {
//             let mut vec = vec![];
//             for el in els {
//                 match el {
//                     Object::Integer(num) => vec.push(*num),
//                     _ => continue,
//                 }
//             }

//             vec = vec.iter().map(|el| el * _const).collect();

//             let mut obj_vec = vec![];
//             for el in vec {
//                 obj_vec.push(Object::Integer(el));
//             }

//             Some(Object::Array(obj_vec))
//         },
//         _ => panic!("Unrecognizable function")
//     }
// }

// fn eval_statement(statement: Statement, env:& mut Env) -> Object {
//     match statement {
//         Statement::Expression(expr) => eval_expr(expr, env),
//         Statement::Let(ident, val) => {
//             let _val = eval_expr(val, env);
//             env.set(ident, _val.clone());
//             _val
//         },
//         Statement::Return(expr) => {
//             Object::Return(Box::new(eval_expr(expr, env)))
//         }
//         _ => panic!("Unidentified statement"),
//     }
// }

// fn eval_statements(stmnts: Vec<Statement>, env: &mut Env) -> Object {
//     let mut result = Object::Null;

//     for stmnt in stmnts {
//         result = eval_statement(stmnt, env);
//         if let &Object::Return(_) = &result {
//             return result;
//         }
//     }
//     result
// }

// fn eval_return(stmnts: Vec<Statement>, env: &mut Env) -> Object {
//     let result = eval_statements(stmnts, env);
//     match result {
//         Object::Return(val) => *val,
//         _ => result
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::parser_mod::Parser::Parser;
//     use crate::lexer_mod::lexer::Lexer;

//     fn eval(input: &str, val: Object) {
//         let mut lexer = Lexer::new(input);
//         let mut parser = Parser::new(lexer);
//         let mut output = parser.parse();
//         let mut env = Env::new();
//         let result = eval_return(output, &mut env);
//         assert_eq!(result, val);
//     }
//     #[test]
//     fn test_numbers() {
//         eval("100;", Object::Integer(100));
//         eval("100 + 100;", Object::Integer(200));
//         eval("100 - 100;", Object::Integer(0));
//         eval("100 * 10;", Object::Integer(1000));
//         eval("100 / 10;", Object::Integer(10));
//         eval("-100;", Object::Integer(-100));
//         eval("-100 + 25 * 23 + 1000;", Object::Integer(1475));
//         eval("-100;", Object::Integer(-100));
//         eval("-100;", Object::Integer(-100));
//     }

//     #[test]
//     fn test_bools() {
//         eval("false;", Object::Boolean(false));
//         eval("true;", Object::Boolean(true));
//         eval("!false;", Object::Boolean(true));
//         eval("!true;", Object::Boolean(false));
//     }
//     #[test]
//     fn test_arrays() {
//         let one = Object::Integer(1);
//         let two = Object::Integer(2);
//         let three = Object::Integer(3);
//         let four = Object::Integer(4);
//         let five = Object::Integer(5);
//         eval("[1, 2, 3, 4, 5];", Object::Array([one, two, three, four, five].to_vec()));

//         let one = Object::Integer(1);
//         let two = Object::Integer(2);
//         let three = Object::Integer(3);
//         let four = Object::Integer(4);
//         let five = Object::Integer(5);
//         eval("[1+1-1, 2+2-2, 3+1-1, 4+0, 5/1];", Object::Array([one, two, three, four, five].to_vec()));
//     }

//     #[test]
//     fn test_built_in() {
//         eval("size([1, 2, 3, 4, 5]);", Object::Integer(5));
//         eval("max([1, 2, 3, 4, 5]);", Object::Integer(5));
//         eval("min([1, 2, 3, 4, 5]);", Object::Integer(1));
//         eval("sum([1, 2, 3, 4, 5]);", Object::Integer(15));
//         eval("constant_product([1, 2], 10);", Object::Array([Object::Integer(10), Object::Integer(20)].to_vec()));
//     }

//     // inmultirea, impartirea cu o constanta,
//     // inmultirea, adunarea a 2 matrici
//     // transpose(arr)
//     // inverse(arr)
//     // print(arr[0]) -> 1, 2, 3, 4, 5
//     // arr[randuri][coloane]
//     // print(arr)
//     // #[test]
//     // fn test_functions() {
//     //     eval("fn sum(a, b) { return a + b; } sum(100, 200);", Object::Integer(300));
//     // }
// }

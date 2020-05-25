use crate::evaluation_mod::env::Env;
use crate::lexer_mod::lexer::Lexer;
use crate::parser_mod::ParseItem::{Expression, Infix, Prefix, Statement};
use crate::parser_mod::Parser::Parser;

#[derive(Clone, PartialEq, Debug)]
pub enum Object {
    Null,
    Integer(i64),
    String(String),
    Boolean(bool),
    Return(Box<Object>),
    Function(String, Vec<String>, Vec<Statement>),
    Array(Vec<Vec<Object>>),
}

fn eval_expr(expression: Expression, env: &mut Env) -> Object {
    match expression {
        Expression::Array(elems) => {
            let mut els: Vec<Vec<Object>> = vec![];
            for row in elems {
                let arr = row.into_iter().map(|expr| eval_expr(expr, env)).collect();
                els.push(arr);
            }
            Object::Array(els)
        }
        Expression::Index(input_array, first_num, second_num) => {
            let arr_name;
            match eval_expr(*input_array, env) {
                Object::String(name) => arr_name = name,
                _ => panic!("Invalid identifier"),
            }
            let mut array = vec![];
            match env.get(arr_name.as_str()) {
                Some(Object::Array(els)) => {
                    let mut row_arr = vec![];
                    for row in els {
                        for col in row {
                            match col {
                                Object::Integer(num) => row_arr.push(num),
                                _ => continue,
                            }
                        }
                    }
                    array.push(row_arr);
                }
                _ => panic!("Unknown array"),
            }

            match *second_num {
                Some(expr) => match (eval_expr(*first_num, env), eval_expr(expr, env)) {
                    (Object::Integer(num1), Object::Integer(num2)) => {
                        if num1 > array.len() as i64 {
                            panic!("Array rows out of range");
                        }
                        if num2 > array[num1 as usize].len() as i64 {
                            panic!("Array column out of range");
                        }
                        Object::Integer(array[num1 as usize][num2 as usize])
                    }
                    _ => panic!("Invalid index"),
                },
                None => match eval_expr(*first_num, env) {
                    Object::Integer(num1) => {
                        if num1 > array.len() as i64 {
                            panic!("Array index out of range");
                        }
                        Object::Integer(array[0][num1 as usize])
                    }
                    _ => panic!("Invalid index"),
                },
            }
        }
        Expression::Integer(num) => Object::Integer(num),
        Expression::Bool(_bool) => Object::Boolean(_bool),
        Expression::Identifier(name) => {
            let value = env
                .get(&name.as_str())
                .expect(format!("{} not found", name).as_str());
            println!("{:?}", value);
            Object::String(name)
        }
        Expression::Function(ident, params, body) => Object::Function(ident, params, body),
        Expression::Call { func, args } => {
            let (params, body) = match *func {
                Expression::Identifier(name) => match env.get(&name) {
                    Some(Object::Function(_, args, body)) => (args, body),
                    _ => {
                        let elems = args.into_iter().map(|expr| eval_expr(expr, env)).collect();
                        return eval_builtin(&name, elems, env).expect("Unrecognized function");
                    }
                },
                _ => panic!("yo, what's that?"),
            };

            if params.len() != args.len() {
                panic!("Wrong number of params");
            }

            let mut env2 = Env::new();
            for (arg, val) in params.into_iter().zip(args.into_iter()) {
                env2.set(arg, eval_expr(val, env));
            }

            eval_return(body, &mut env2)
        }
        Expression::Prefix(Prefix::Not, expression) => match eval_expr(*expression, env) {
            Object::Boolean(_bool) => Object::Boolean(!_bool),
            _ => panic!("Can use ! operator only for booleans"),
        },
        Expression::Prefix(Prefix::Minus, expression) => match eval_expr(*expression, env) {
            Object::Integer(num) => Object::Integer(-num),
            _ => panic!("Can use the '-' operator only for numbers"),
        },
        Expression::Infix(Infix::Plus, lhs, rhs) => {
            match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
                (Object::Integer(num1), Object::Integer(num2)) => Object::Integer(num1 + num2),
                _ => panic!("Can only add integer literals"),
            }
        }
        Expression::Infix(Infix::Minus, lhs, rhs) => {
            match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
                (Object::Integer(num1), Object::Integer(num2)) => Object::Integer(num1 - num2),
                _ => panic!("Can only subtract integer literals"),
            }
        }
        Expression::Infix(Infix::Divide, lhs, rhs) => {
            match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
                (Object::Integer(num1), Object::Integer(num2)) => Object::Integer(num1 / num2),
                _ => panic!("Can only divide integer literals"),
            }
        }
        Expression::Infix(Infix::Multiply, lhs, rhs) => {
            match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
                (Object::Integer(num1), Object::Integer(num2)) => Object::Integer(num1 * num2),
                _ => panic!("Can only multiply integer literals"),
            }
        }
        Expression::Infix(Infix::Equal, lhs, rhs) => {
            match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
                (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 == num2),
                _ => panic!("Can only compare integer literals"),
            }
        }
        Expression::Infix(Infix::NotEqual, lhs, rhs) => {
            match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
                (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 != num2),
                _ => panic!("Can only compare integer literals"),
            }
        }
        Expression::Infix(Infix::MoreThanAndEqual, lhs, rhs) => {
            match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
                (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 >= num2),
                _ => panic!("Can only compare integer literals"),
            }
        }
        Expression::Infix(Infix::MoreThan, lhs, rhs) => {
            match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
                (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 > num2),
                _ => panic!("Can only compare integer literals"),
            }
        }
        Expression::Infix(Infix::LessThanAndEqual, lhs, rhs) => {
            match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
                (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 <= num2),
                _ => panic!("Can only compare integer literals"),
            }
        }
        Expression::Infix(Infix::LessThan, lhs, rhs) => {
            match (eval_expr(*lhs, env), eval_expr(*rhs, env)) {
                (Object::Integer(num1), Object::Integer(num2)) => Object::Boolean(num1 < num2),
                _ => panic!("Can only compare integer literals"),
            }
        }
        Expression::Infix(Infix::Assign, lhs, rhs) => {
            let expr_lhs = lhs.clone();
            let expr_rhs = rhs.clone();
            match (*expr_lhs, eval_expr(*expr_rhs, env)) {
                (Expression::Index(arr, first_index, second_index), Object::Integer(number)) => {
                    let arr_copy = arr.clone();
                    let arr_name;
                    match eval_expr(*arr_copy, env) {
                        Object::String(_name) => arr_name = _name,
                        _ => panic!("Unrecognized Identifier"),
                    }

                    let first_index_clone = first_index.clone();
                    let second_index_clone = second_index.clone();

                    match eval_expr(Expression::Index(arr, first_index, second_index), env) {
                        Object::Integer(num) => {
                            let array_obj = env.get(arr_name.as_str());
                            let row;
                            let col;
                            match *second_index_clone {
                                Some(expr) => {
                                    let obj = eval_expr(expr, env);
                                    match obj {
                                        Object::Integer(num) => {
                                            row = num;
                                        }
                                        _ => panic!("Invalid Index"),
                                    }
                                }
                                _ => row = 0,
                            }
                            match eval_expr(*first_index_clone, env) {
                                Object::Integer(num) => {
                                    col = num;
                                }
                                _ => panic!("Invalid Index"),
                            }

                            match array_obj {
                                Some(Object::Array(els)) => {
                                    let mut final_arr = vec![];
                                    for i in 0..els.len() {
                                        let mut row_arr = vec![];
                                        for j in 0..els[i].len() {
                                            if i == row as usize && j == col as usize {
                                                row_arr.push(Object::Integer(number));
                                                continue;
                                            }
                                            match els[i][j] {
                                                Object::Integer(num) => {
                                                    row_arr.push(Object::Integer(num));
                                                }
                                                _ => (),
                                            }
                                        }
                                        final_arr.push(row_arr);
                                    }
                                    env.set(arr_name, Object::Array(final_arr));
                                    return Object::Integer(0);
                                }
                                _ => {
                                    panic!("Unexpected Identifier");
                                }
                            }
                        }
                        _ => panic!("Invalid Index"),
                    }
                }
                _ => (),
            }

            let lhs_name;
            match eval_expr(*lhs, env) {
                Object::String(name) => lhs_name = name,
                _ => panic!("Invalid assignment"),
            }
            match eval_expr(*rhs, env) {
                Object::Integer(num2) => {
                    env.set(lhs_name, Object::Integer(num2));
                    Object::Integer(num2)
                }
                Object::Boolean(_bool) => {
                    env.set(lhs_name, Object::Boolean(_bool));
                    Object::Boolean(_bool)
                }
                Object::Array(els) => {
                    env.set(lhs_name, Object::Array(els));
                    Object::Integer(0)
                }
                _ => panic!("Invalid assignment"),
            }
        }
        _ => Object::Null,
    }
}

fn eval_builtin(name: &str, args: Vec<Object>, env: &mut Env) -> Option<Object> {
    match (name, args.as_slice()) {
        ("size", [Object::Array(els)]) => {
            let mut total = 0;
            for row in els {
                total += row.len();
            }
            Some(Object::Integer(total as i64))
        }
        ("max", [Object::Array(els)]) => {
            let mut max = i64::min_value();
            for row in els {
                for el in row {
                    match el {
                        Object::Integer(num) => {
                            if (*num > max) {
                                max = *num;
                            }
                        }
                        _ => continue,
                    }
                }
            }
            Some(Object::Integer(max))
        }
        ("min", [Object::Array(els)]) => {
            let mut min = i64::max_value();
            for row in els {
                for el in row {
                    match el {
                        Object::Integer(num) => {
                            if (*num < min) {
                                min = *num;
                            }
                        }
                        _ => continue,
                    }
                }
            }
            Some(Object::Integer(min))
        }
        ("sum", [Object::Array(els)]) => {
            let mut sum: i64 = 0;
            for row in els {
                for el in row {
                    match el {
                        Object::Integer(num) => sum += num,
                        _ => continue,
                    }
                }
            }
            Some(Object::Integer(sum))
        }
        ("constant_product", [Object::Array(els), Object::Integer(_const)]) => {
            let mut vec = vec![];
            for row in els {
                let mut row_arr = vec![];
                for el in row {
                    match el {
                        Object::Integer(num) => row_arr.push(Object::Integer(*num * _const)),
                        _ => continue,
                    }
                }
                vec.push(row_arr);
            }

            Some(Object::Array(vec))
        }
        ("dot_product", [Object::Array(first_arr), Object::Array(second_arr)]) => {
            if !check_array_size(first_arr) || !check_array_size(second_arr) {
                panic!("Array's rows need to be the same size");
            }
            if first_arr[0].len() != second_arr.len() {
                panic!("The number of columns of the 1st matrix must equal the number of rows of the 2nd matrix.");
            }

            let mut vec: Vec<Vec<Object>> = vec![];

            for i in 0..first_arr.len() {
                let mut arr = vec![];
                for j in 0..second_arr[0].len() {
                    arr.push(Object::Integer(0));
                }
                vec.push(arr);
            }

            for row in 0..first_arr.len() {
                for col in 0..second_arr[0].len() {
                    for k in 0..first_arr[0].len() {
                        match (&first_arr[row][k], &second_arr[k][col]) {
                            (Object::Integer(num1), Object::Integer(num2)) => {
                                vec[row][col] = Object::Integer(num1 * num2)
                            }
                            _ => (),
                        }
                    }
                }
            }

            Some(Object::Array(vec))
        }
        ("constant_division", [Object::Array(els), Object::Integer(_const)]) => {
            let mut vec = vec![];
            for row in els {
                let mut row_arr = vec![];
                for el in row {
                    match el {
                        Object::Integer(num) => row_arr.push(Object::Integer(*num / _const)),
                        _ => continue,
                    }
                }
                vec.push(row_arr);
            }
            Some(Object::Array(vec))
        }
        ("transpose_matrix", [Object::Array(els)]) => {
            check_array_size(&els);
            let mut vec = vec![];
            for col in (0..els[0].len()) {
                let mut arr = vec![];
                for row in 0..els.len() {
                    match els[row][col] {
                        Object::Integer(num) => arr.push(Object::Integer(num)),
                        _ => continue,
                    }
                }
                vec.push(arr);
            }
            Some(Object::Array(vec))
        }
        ("add_matrix", [Object::Array(first_arr), Object::Array(second_arr)]) => {
            if !check_array_size(&first_arr) || !check_array_size(&second_arr) {
                panic!("Array's rows need to be the same size");
            }

            if (first_arr.len() != second_arr.len()) || (first_arr[0].len() != second_arr[0].len())
            {
                panic!("Arrays should be the same size");
            }

            let mut result = vec![];

            for row in 0..first_arr.len() {
                let mut arr = vec![];
                for col in 0..first_arr[0].len() {
                    match (&first_arr[row][col], &second_arr[row][col]) {
                        (Object::Integer(num1), Object::Integer(num2)) => {
                            arr.push(Object::Integer(num1 + num2))
                        }
                        _ => continue,
                    }
                }
                result.push(arr);
            }

            Some(Object::Array(result))
        }
        ("diff_matrix", [Object::Array(first_arr), Object::Array(second_arr)]) => {
            if !check_array_size(first_arr) || !check_array_size(second_arr) {
                panic!("Array's rows need to be the same size");
            }
            if (first_arr.len() != second_arr.len()) || (first_arr[0].len() != second_arr[0].len())
            {
                panic!("Arrays should to be the same size");
            }

            let mut result = vec![];

            for row in 0..first_arr.len() {
                let mut arr = vec![];
                for col in 0..first_arr[0].len() {
                    match (&first_arr[row][col], &second_arr[row][col]) {
                        (Object::Integer(num1), Object::Integer(num2)) => {
                            arr.push(Object::Integer(num1 - num2))
                        }
                        _ => (),
                    }
                }
                result.push(arr);
            }
            Some(Object::Array(result))
        }
        ("get_row", [Object::Integer(index), Object::Array(els)]) => {
            if *index as usize > els.len() || *index <= 0 {
                panic!("Invalid index");
            }

            let mut result = vec![];
            let row = *index as usize - 1;

            for el in 0..els[row - 1].len() {
                match els[row][el] {
                    Object::Integer(int) => {
                        result.push(Object::Integer(int));
                    }
                    _ => (),
                }
            }
            Some(Object::Array(vec![result]))
        }
        ("get_col", [Object::Integer(index), Object::Array(els)]) => {
            if !check_array_size(els) {
                panic!("Array's rows need to be the same size");
            }

            if *index as usize > els[0].len() || *index <= 0 {
                panic!("Invalid index");
            }

            let mut result = vec![];
            let col = *index as usize - 1;

            for el in 0..els.len() {
                match els[el][col] {
                    Object::Integer(int) => {
                        result.push(Object::Integer(int));
                    }
                    _ => (),
                }
            }
            Some(Object::Array(vec![result]))
        }
        ("print", [Object::Array(els)]) => {
            let mut vec = vec![];
            for row in els {
                let mut row_arr = vec![];
                for el in row {
                    match el {
                        Object::Integer(num) => row_arr.push(Object::Integer(*num)),
                        _ => continue,
                    }
                }
                vec.push(row_arr);
                println!("{:?}", vec);
            }

            Some(Object::Array(vec))
        }
        ("print", [Object::Integer(number)]) => {
            println!("{}", number);
            Some(Object::Integer(*number))
        }
        ("print", [Object::String(name)]) => {
            let obj = env.get(name.as_str());
            println!("{:?} {:?}", name, obj);
            match &obj {
                Some(object) => println!("{:?}", *object),
                None => panic!("Unrecognized Identifier"),
            }
            obj
        }
        _ => panic!("Unrecognizable function"),
    }
}
fn check_array_size(arr: &Vec<Vec<Object>>) -> bool {
    for index in 0..arr.len() - 1 {
        if (arr[index].len() != arr[index + 1].len()) {
            return false;
        }
    }
    true
}
fn eval_statement(statement: Statement, env: &mut Env) -> Object {
    match statement {
        Statement::Expression(expr) => eval_expr(expr, env),
        Statement::Let(ident, val) => {
            let _val = eval_expr(val, env);
            env.set(ident, _val.clone());
            _val
        }
        Statement::Return(expr) => Object::Return(Box::new(eval_expr(expr, env))),
        _ => panic!("Unidentified statement"),
    }
}

pub fn eval_statements(stmnts: Vec<Statement>, env: &mut Env) -> Object {
    let mut result = Object::Null;

    for stmnt in stmnts {
        result = eval_statement(stmnt, env);
        if let &Object::Return(_) = &result {
            return result;
        }
    }
    result
}

fn eval_return(stmnts: Vec<Statement>, env: &mut Env) -> Object {
    let result = eval_statements(stmnts, env);
    match result {
        Object::Return(val) => *val,
        _ => result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer_mod::lexer::Lexer;
    use crate::parser_mod::Parser::Parser;

    fn eval(input: &str, val: Object) {
        let mut lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let mut output = parser.parse();
        let mut env = Env::new();
        let result = eval_return(output, &mut env);
        assert_eq!(result, val);
    }
    #[test]
    fn test_numbers() {
        eval("100;", Object::Integer(100));
        eval("100 + 100;", Object::Integer(200));
        eval("100 - 100;", Object::Integer(0));
        eval("100 * 10;", Object::Integer(1000));
        eval("100 / 10;", Object::Integer(10));
        eval("-100;", Object::Integer(-100));
        eval("-100 + 25 * 23 + 1000;", Object::Integer(1475));
        eval("-100;", Object::Integer(-100));
        eval("-100;", Object::Integer(-100));
    }

    #[test]
    fn test_bools() {
        eval("false;", Object::Boolean(false));
        eval("true;", Object::Boolean(true));
        eval("!false;", Object::Boolean(true));
        eval("!true;", Object::Boolean(false));
    }
    #[test]
    fn test_arrays() {
        let one = Object::Integer(1);
        let two = Object::Integer(2);
        let three = Object::Integer(3);
        let four = Object::Integer(4);
        let five = Object::Integer(5);
        eval(
            "[{1, 2, 3, 4, 5}];",
            Object::Array([[one, two, three, four, five].to_vec()].to_vec()),
        );

        let one = Object::Integer(1);
        let two = Object::Integer(2);
        let three = Object::Integer(3);
        let four = Object::Integer(4);
        let five = Object::Integer(5);
        eval(
            "[{1+1-1, 2+2-2, 3+1-1, 4+0, 5/1} {1, 2, 3, 4, 5}];",
            Object::Array(
                [
                    [one, two, three, four, five].to_vec(),
                    [
                        Object::Integer(1),
                        Object::Integer(2),
                        Object::Integer(3),
                        Object::Integer(4),
                        Object::Integer(5),
                    ]
                    .to_vec(),
                ]
                .to_vec(),
            ),
        );
    }

    #[test]
    fn test_built_in() {
        eval("size([{1, 2, 3, 4, 5}]);", Object::Integer(5));
        eval(
            "size([{1, 2, 3, 4, 5} {10, 12, 123, 155, 166} {100, 99, 98, 97, 96}]);",
            Object::Integer(15),
        );
        eval(
            "max([{1, 2, 3, 4, 5} {10, 12, 123, 155, 166} {100, 99, 98, 97, 96}]);",
            Object::Integer(166),
        );
        eval(
            "min([{1, 2, 3, 4, 5} {10, 12, 123, 155, 166} {100, 99, 98, 97, 96}]);",
            Object::Integer(1),
        );
        eval(
            "sum([{1, 2, 3, 4, 5} {3, 4, 1, 2, 0} {10, 10, 10, 10, 10} ]);",
            Object::Integer(75),
        );
        eval(
            "constant_product([{1, 2}], 10);",
            Object::Array([[Object::Integer(10), Object::Integer(20)].to_vec()].to_vec()),
        );
        // eval(
        //     "dot_product([{2, 4}], [{1, 2} {3, 4}]);",
        //     Object::Array([[Object::Integer(58), Object::Integer(64)].to_vec()].to_vec()),
        // );
        eval(
            "transpose_matrix([{1, 2, 3} {4, 5, 6}]);",
            Object::Array(
                [
                    [Object::Integer(1), Object::Integer(4)].to_vec(),
                    [Object::Integer(2), Object::Integer(5)].to_vec(),
                    [Object::Integer(3), Object::Integer(6)].to_vec(),
                ]
                .to_vec(),
            ),
        );
        eval(
            "add_matrix([{1, 2, 3, 4, 5} {1, 2, 3, 4, 5}], [{10, 10, 10, 10, 10} { 10, 10, 10, 10, 10 }]);",
            Object::Array(
                [
                    [Object::Integer(11), Object::Integer(12), Object::Integer(13), Object::Integer(14), Object::Integer(15)].to_vec(),
                    [Object::Integer(11), Object::Integer(12), Object::Integer(13), Object::Integer(14), Object::Integer(15)].to_vec(),
                ].to_vec(),
            )
        );
        eval(
            "diff_matrix([{1, 2, 3, 4, 5} {1, 2, 3, 4, 5}], [{10, 10, 10, 10, 10} { 10, 10, 10, 10, 10 }]);",
            Object::Array(
                [
                    [Object::Integer(-9), Object::Integer(-8), Object::Integer(-7), Object::Integer(-6), Object::Integer(-5)].to_vec(),
                    [Object::Integer(-9), Object::Integer(-8), Object::Integer(-7), Object::Integer(-6), Object::Integer(-5)].to_vec(),
                ].to_vec(),
            )
        );
        eval(
            "get_row(1, [{1, 2, 3, 4, 5} {1, 2, 3, 4, 5}])",
            Object::Array(
                [[
                    Object::Integer(1),
                    Object::Integer(2),
                    Object::Integer(3),
                    Object::Integer(4),
                    Object::Integer(5),
                ]
                .to_vec()]
                .to_vec(),
            ),
        );
        eval(
            "get_col(2, [{1, 2, 3, 4, 5} {1, 2, 3, 4, 5}])",
            Object::Array([[Object::Integer(2), Object::Integer(2)].to_vec()].to_vec()),
        );
    }
}

// transpose randuri print
// index, returnam si sa schimbam

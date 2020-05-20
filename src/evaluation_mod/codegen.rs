extern crate llvm_sys as llvm;
use crate::parser_mod::ParseItem;
use crate::parser_mod::ParseItem::{Expression, Infix, Prefix, Statement};
use llvm::prelude::*;
use std::collections::{HashMap, HashSet};
use std::ffi::CString;

use std::ptr;

pub enum Object {
    Integer(i64),
    Boolean(bool),
    Print,
}

pub unsafe fn generate_code(input: Vec<ParseItem::Statement>) {
    let context = llvm::core::LLVMContextCreate();
    let module = llvm::core::LLVMModuleCreateWithName(b"example_module\0".as_ptr() as *const _);
    let builder = llvm::core::LLVMCreateBuilderInContext(context);
    let int_type = llvm::core::LLVMInt64TypeInContext(context);
    let function_type = llvm::core::LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);
    let function =
        llvm::core::LLVMAddFunction(module, b"main\0".as_ptr() as *const _, function_type);
    let entry_name = CString::new("entry").unwrap();
    let bb = llvm::core::LLVMAppendBasicBlockInContext(context, function, entry_name.as_ptr());
    llvm::core::LLVMPositionBuilderAtEnd(builder, bb);
    let mut names = HashMap::new();
    let mut built_ins: HashMap<&'static str, *mut llvm::LLVMValue> = HashMap::new();
    built_ins.insert("printf", create_printf(module));
    codegen(input, module, context, builder, function, names, built_ins);
}

unsafe fn codegen(
    input: Vec<ParseItem::Statement>,
    module: LLVMModuleRef,
    context: LLVMContextRef,
    builder: LLVMBuilderRef,
    function: LLVMValueRef,
    mut names: HashMap<String, LLVMValueRef>,
    mut built_ins: HashMap<&'static str, *mut llvm::LLVMValue>,
) {
    insert_allocations(context, builder, &mut names, &input);

    let int_type = llvm::core::LLVMInt64TypeInContext(context);
    let zero = llvm::core::LLVMConstInt(int_type, 0, 0);

    let mut return_value = zero;
    for expr in input {
        return_value = codegen_expr(context, builder, function, &mut names, expr, &built_ins);
    }
    llvm::core::LLVMBuildRet(builder, return_value);

    let out_file = CString::new("out.ll").unwrap();
    llvm::core::LLVMPrintModuleToFile(module, out_file.as_ptr(), ptr::null_mut());

    llvm::core::LLVMDisposeBuilder(builder);
    llvm::core::LLVMDisposeModule(module);
    llvm::core::LLVMContextDispose(context);
}

unsafe fn insert_allocations(
    context: LLVMContextRef,
    builder: LLVMBuilderRef,
    names: &mut HashMap<String, LLVMValueRef>,
    exprs: &[ParseItem::Statement],
) {
    let mut variable_names = HashSet::new();
    for expr in exprs {
        match *expr {
            ParseItem::Statement::Let(ref name, _) => {
                variable_names.insert(name);
            }

            _ => {}
        }
    }

    for variable_name in variable_names {
        let int_type = llvm::core::LLVMInt64TypeInContext(context);
        let name = CString::new(variable_name.as_bytes()).unwrap();
        let pointer = llvm::core::LLVMBuildAlloca(builder, int_type, name.as_ptr());

        names.insert(variable_name.to_owned(), pointer);
    }
}

pub fn call_function(
    builder: *mut llvm::LLVMBuilder,
    function: *mut llvm::LLVMValue,
    mut args: Vec<*mut llvm::LLVMValue>,
    name: &str,
) -> *mut llvm::LLVMValue {
    unsafe {
        llvm::core::LLVMBuildCall(
            builder,
            function,
            args.as_mut_ptr(),
            args.len() as u32,
            CString::new(name).unwrap().as_ptr(),
        )
    }
}

pub fn pointer_type() -> *mut llvm::LLVMType {
    unsafe { llvm::core::LLVMPointerType(llvm::core::LLVMInt64Type(), 0) }
}

pub unsafe fn create_printf(module: *mut llvm::LLVMModule) -> *mut llvm::LLVMValue {
    let mut printf_args_type_list = vec![pointer_type()];
    let printf_type =
        llvm::core::LLVMFunctionType(pointer_type(), printf_args_type_list.as_mut_ptr(), 0, 1);

    llvm::core::LLVMAddFunction(
        module,
        CString::new("printf").unwrap().as_ptr(),
        printf_type,
    )
}

unsafe fn codegen_expr(
    context: LLVMContextRef,
    builder: LLVMBuilderRef,
    func: LLVMValueRef,
    names: &mut HashMap<String, LLVMValueRef>,
    expr: ParseItem::Statement,
    mut built_ins: &HashMap<&'static str, *mut llvm::LLVMValue>,
) -> LLVMValueRef {
    match expr {
        Statement::Expression(Expression::Integer(int_literal)) => {
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            let res = llvm::core::LLVMConstInt(int_type, int_literal as u64, 0);
            res
        }

        Statement::Expression(Expression::Infix(Infix::Plus, lhs, rhs)) => {
            let lhs = codegen_expr(
                context,
                builder,
                func,
                names,
                Statement::Expression(*lhs),
                built_ins,
            );
            let rhs = codegen_expr(
                context,
                builder,
                func,
                names,
                Statement::Expression(*rhs),
                built_ins,
            );

            let name = CString::new("addtmp").unwrap();
            let res = llvm::core::LLVMBuildAdd(builder, lhs, rhs, name.as_ptr());
            call_function(builder, built_ins["printf"], vec![res], "");
            res
        }

        Statement::Expression(Expression::Infix(Infix::Minus, lhs, rhs)) => {
            let lhs = codegen_expr(
                context,
                builder,
                func,
                names,
                Statement::Expression(*lhs),
                built_ins,
            );
            let rhs = codegen_expr(
                context,
                builder,
                func,
                names,
                Statement::Expression(*rhs),
                built_ins,
            );

            let name = CString::new("subtmp").unwrap();
            let res = llvm::core::LLVMBuildSub(builder, lhs, rhs, name.as_ptr());
            call_function(builder, built_ins["printf"], vec![res], "");
            res
        }

        Statement::Let(name, expr) => {
            let new_value = codegen_expr(
                context,
                builder,
                func,
                names,
                Statement::Expression(expr),
                built_ins,
            );
            let pointer = names.get(&name).unwrap();
            llvm::core::LLVMBuildStore(builder, new_value, *pointer);
            new_value
        }

        Statement::Expression(Expression::Infix(Infix::Assign, name, expr)) => {
            let new_value = codegen_expr(
                context,
                builder,
                func,
                names,
                Statement::Expression(*expr),
                built_ins,
            );
            let ident_name = match *name {
                Expression::Identifier(arg) => arg,
                _ => "".to_string(),
            };
            let pointer = names.get(&ident_name).unwrap();
            llvm::core::LLVMBuildStore(builder, new_value, *pointer);
            new_value
        }
        Statement::Expression(Expression::Identifier(name)) => {
            let pointer = names.get(&name).unwrap();
            let name = CString::new(name).unwrap();
            llvm::core::LLVMBuildLoad(builder, *pointer, name.as_ptr())
        }

        Statement::Expression(Expression::Infix(Infix::Divide, lhs, rhs)) => {
            let lhs = codegen_expr(
                context,
                builder,
                func,
                names,
                Statement::Expression(*lhs),
                built_ins,
            );
            let rhs = codegen_expr(
                context,
                builder,
                func,
                names,
                Statement::Expression(*rhs),
                built_ins,
            );

            let name = CString::new("divtmp").unwrap();
            let res = llvm::core::LLVMBuildUDiv(builder, lhs, rhs, name.as_ptr());
            call_function(builder, built_ins["printf"], vec![res], "");
            res
        }

        Statement::Expression(Expression::Infix(Infix::Multiply, lhs, rhs)) => {
            let lhs = codegen_expr(
                context,
                builder,
                func,
                names,
                Statement::Expression(*lhs),
                built_ins,
            );
            let rhs = codegen_expr(
                context,
                builder,
                func,
                names,
                Statement::Expression(*rhs),
                built_ins,
            );

            let name = CString::new("multmp").unwrap();
            let res = llvm::core::LLVMBuildMul(builder, lhs, rhs, name.as_ptr());
            call_function(builder, built_ins["printf"], vec![res], "");
            res
        }

        Statement::Expression(Expression::If(condition, then_body, else_body)) => {
            let condition_value = codegen_expr(
                context,
                builder,
                func,
                names,
                Statement::Expression(*condition),
                built_ins,
            );
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            let zero = llvm::core::LLVMConstInt(int_type, 0, 0);

            let name = CString::new("is_nonzero").unwrap();
            let is_nonzero = llvm::core::LLVMBuildICmp(
                builder,
                llvm::LLVMIntPredicate::LLVMIntNE,
                condition_value,
                zero,
                name.as_ptr(),
            );

            let entry_name = CString::new("entry").unwrap();
            let then_block =
                llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());
            let else_block =
                llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());
            let merge_block =
                llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());

            llvm::core::LLVMBuildCondBr(builder, is_nonzero, then_block, else_block);

            llvm::core::LLVMPositionBuilderAtEnd(builder, then_block);
            let mut then_return = zero;
            for expr in then_body {
                then_return = codegen_expr(context, builder, func, names, expr, built_ins);
            }
            llvm::core::LLVMBuildBr(builder, merge_block);
            let then_block = llvm::core::LLVMGetInsertBlock(builder);

            llvm::core::LLVMPositionBuilderAtEnd(builder, else_block);
            let mut else_return = zero;
            match else_body {
                Some(exprsns) => {
                    for expr in exprsns {
                        else_return = codegen_expr(context, builder, func, names, expr, built_ins);
                    }
                }
                _ => else_return = zero,
            };
            llvm::core::LLVMBuildBr(builder, merge_block);
            let else_block = llvm::core::LLVMGetInsertBlock(builder);

            llvm::core::LLVMPositionBuilderAtEnd(builder, merge_block);
            let phi_name = CString::new("iftmp").unwrap();
            let phi = llvm::core::LLVMBuildPhi(builder, int_type, phi_name.as_ptr());

            let mut values = vec![then_return, else_return];
            let mut blocks = vec![then_block, else_block];

            llvm::core::LLVMAddIncoming(phi, values.as_mut_ptr(), blocks.as_mut_ptr(), 2);
            call_function(builder, built_ins["printf"], vec![phi], "");
            phi
        }
        _ => llvm::core::LLVMConstInt(llvm::core::LLVMInt64TypeInContext(context), 0, 0),
    }
}

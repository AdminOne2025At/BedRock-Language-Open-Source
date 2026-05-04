// IR (Intermediate Representation) Generation
use crate::ast::*;
use crate::error::Result;

#[derive(Debug, Clone)]
pub enum IRInstr {
    Load(u32, u32),
    Store(u32, u32),
    BinOp(u32, String, u32, u32),
    UnOp(u32, String, u32),
    Call(u32, String, Vec<u32>),
    Label(String),
    Jump(String),
    JumpIf(u32, String),
    Return(Option<u32>),
}

pub fn lower(program: &Program) -> Result<Vec<IRInstr>> {
    let mut ir = Vec::new();
    let mut var_counter = 0;

    for item in &program.items {
        match item {
            Item::Function(func) => {
                ir.push(IRInstr::Label(func.name.clone()));
                for stmt in &func.body {
                    lower_statement(stmt, &mut ir, &mut var_counter)?;
                }
            }
            _ => {}
        }
    }

    Ok(ir)
}

fn lower_statement(
    stmt: &Statement,
    ir: &mut Vec<IRInstr>,
    var_counter: &mut u32,
) -> Result<()> {
    match stmt {
        Statement::Return(expr) => {
            if let Some(e) = expr {
                let val = lower_expression(e, ir, var_counter)?;
                ir.push(IRInstr::Return(Some(val)));
            } else {
                ir.push(IRInstr::Return(None));
            }
        }
        Statement::Expression(expr) => {
            let _ = lower_expression(expr, ir, var_counter)?;
        }
        Statement::Block(stmts) => {
            for s in stmts {
                lower_statement(s, ir, var_counter)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn lower_expression(
    expr: &Expression,
    ir: &mut Vec<IRInstr>,
    var_counter: &mut u32,
) -> Result<u32> {
    match expr {
        Expression::Literal(Literal::Integer(n)) => {
            let result = *var_counter;
            *var_counter += 1;
            ir.push(IRInstr::Load(result, *n as u32));
            Ok(result)
        }
        Expression::BinaryOp { left, op, right } => {
            let left_val = lower_expression(left, ir, var_counter)?;
            let right_val = lower_expression(right, ir, var_counter)?;
            let result = *var_counter;
            *var_counter += 1;
            let op_str = match op {
                BinaryOp::Add => "add",
                BinaryOp::Sub => "sub",
                BinaryOp::Mul => "mul",
                BinaryOp::Div => "div",
                _ => "unknown",
            };
            ir.push(IRInstr::BinOp(result, op_str.to_string(), left_val, right_val));
            Ok(result)
        }
        Expression::Call { func, args } => {
            if let Expression::Identifier(func_name) = &**func {
                let arg_vals: Result<Vec<u32>> =
                    args.iter().map(|arg| lower_expression(arg, ir, var_counter)).collect();
                let result = *var_counter;
                *var_counter += 1;
                ir.push(IRInstr::Call(result, func_name.clone(), arg_vals?));
                Ok(result)
            } else {
                Ok(0)
            }
        }
        _ => Ok(0),
    }
}

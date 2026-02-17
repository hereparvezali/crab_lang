use std::collections::HashMap;

use crate::parser::{Expr, Op, Stmt};

pub struct CodeGen {
    output: String,
    vars: HashMap<String, i64>,
    stack_offset: i64,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            vars: HashMap::new(),
            stack_offset: 0,
        }
    }

    fn emit(&mut self, line: &str) {
        self.output.push_str(line);
        self.output.push('\n');
    }

    fn emit_indent(&mut self, line: &str) {
        self.output.push_str("    ");
        self.output.push_str(line);
        self.output.push('\n');
    }

    pub fn generate(mut self, stmts: &[Stmt]) -> String {
        // Data section (empty for now, but needed for future string literals etc.)
        self.emit("section .data");
        self.emit("");

        // BSS section for uninitialized data
        self.emit("section .bss");
        self.emit("");

        // Text section
        self.emit("section .text");
        self.emit("global _start");
        self.emit("");

        self.emit("_start:");
        // Set up stack frame
        self.emit_indent("push rbp");
        self.emit_indent("mov rbp, rsp");

        // Reserve stack space for variables
        // Count how many let statements we have
        let var_count = stmts
            .iter()
            .filter(|s| matches!(s, Stmt::Let(_, _)))
            .count();

        if var_count > 0 {
            // Align to 16 bytes for ABI compliance
            let stack_space = ((var_count * 8 + 15) / 16) * 16;
            self.emit_indent(&format!("sub rsp, {}", stack_space));
        }

        self.emit("");

        // Generate code for each statement
        for stmt in stmts {
            self.gen_stmt(stmt);
        }

        // Default exit with code 0 if no exit statement was encountered
        self.emit("");
        self.emit_indent("; default exit");
        self.emit_indent("mov rax, 60");
        self.emit_indent("xor rdi, rdi");
        self.emit_indent("syscall");

        self.output
    }

    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let(name, expr) => {
                self.emit_indent(&format!("; let {} = ...", name));

                // Generate code for the expression, result will be in rax
                self.gen_expr(expr);

                // Allocate stack space for this variable
                self.stack_offset -= 8;
                self.vars.insert(name.clone(), self.stack_offset);

                // Store the result on the stack
                self.emit_indent(&format!("mov [rbp{}], rax", self.stack_offset));
                self.emit("");
            }
            Stmt::Exit(expr) => {
                self.emit_indent("; exit");

                // Generate code for the expression, result will be in rax
                self.gen_expr(expr);

                // syscall: exit(rax)
                self.emit_indent("mov rdi, rax");
                self.emit_indent("mov rax, 60");
                self.emit_indent("syscall");
                self.emit("");
            }
        }
    }

    fn gen_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Num(n) => {
                self.emit_indent(&format!("mov rax, {}", n));
            }
            Expr::Var(name) => {
                let offset = self
                    .vars
                    .get(name)
                    .unwrap_or_else(|| panic!("undefined variable: {}", name));
                self.emit_indent(&format!("mov rax, [rbp{}]", offset));
            }
            Expr::BinOp(left, op, right) => {
                // Evaluate right side first and push onto stack
                self.gen_expr(right);
                self.emit_indent("push rax");

                // Evaluate left side (result in rax)
                self.gen_expr(left);

                // Pop right side into rbx
                self.emit_indent("pop rbx");

                // Perform the operation
                match op {
                    Op::Add => {
                        self.emit_indent("add rax, rbx");
                    }
                    Op::Sub => {
                        self.emit_indent("sub rax, rbx");
                    }
                    Op::Mul => {
                        self.emit_indent("imul rax, rbx");
                    }
                    Op::Div => {
                        // For signed division:
                        // cqo sign-extends rax into rdx:rax
                        // idiv rbx divides rdx:rax by rbx, quotient in rax, remainder in rdx
                        self.emit_indent("cqo");
                        self.emit_indent("idiv rbx");
                    }
                }
            }
        }
    }
}

impl Default for CodeGen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_simple_exit() {
        let source = "exit 42;";
        let tokens = Lexer::new(source).tokenize();
        let stmts = Parser::new(tokens).parse();
        let asm = CodeGen::new().generate(&stmts);

        assert!(asm.contains("mov rax, 42"));
        assert!(asm.contains("mov rdi, rax"));
        assert!(asm.contains("mov rax, 60"));
        assert!(asm.contains("syscall"));
    }

    #[test]
    fn test_let_and_exit() {
        let source = "let x = 10; exit x;";
        let tokens = Lexer::new(source).tokenize();
        let stmts = Parser::new(tokens).parse();
        let asm = CodeGen::new().generate(&stmts);

        assert!(asm.contains("mov rax, 10"));
        assert!(asm.contains("mov [rbp-8], rax"));
        assert!(asm.contains("mov rax, [rbp-8]"));
    }

    #[test]
    fn test_arithmetic() {
        let source = "exit 2 + 3 * 4;";
        let tokens = Lexer::new(source).tokenize();
        let stmts = Parser::new(tokens).parse();
        let asm = CodeGen::new().generate(&stmts);

        // Should contain multiplication and addition operations
        assert!(asm.contains("imul rax, rbx"));
        assert!(asm.contains("add rax, rbx"));
    }
}

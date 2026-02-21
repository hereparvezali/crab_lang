use std::collections::HashMap;

use crate::parser::{Expr, Op, Stmt};

pub struct CodeGen {
    output: String,
    vars: HashMap<String, i64>,
    stack_offset: i64,
    label_counter: usize,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            vars: HashMap::new(),
            stack_offset: 0,
            label_counter: 0,
        }
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!(".{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
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
            Stmt::If(cond, then_body, elif_branches, else_body) => {
                let end_label = self.new_label("if_end");

                // Generate condition for if
                self.emit_indent("; if condition");
                self.gen_expr(cond);
                self.emit_indent("cmp rax, 0");

                if elif_branches.is_empty() && else_body.is_none() {
                    // Simple if without elif or else
                    self.emit_indent(&format!("je {}", end_label));
                    self.emit_indent("; then block");
                    for stmt in then_body {
                        self.gen_stmt(stmt);
                    }
                } else {
                    // If with elif and/or else branches
                    let mut next_label = self.new_label("elif");
                    self.emit_indent(&format!("je {}", next_label));

                    // Then block
                    self.emit_indent("; then block");
                    for stmt in then_body {
                        self.gen_stmt(stmt);
                    }
                    self.emit_indent(&format!("jmp {}", end_label));

                    // Elif branches
                    for (elif_cond, elif_body) in elif_branches {
                        self.emit(&format!("{}:", next_label));
                        next_label = self.new_label("elif");

                        self.emit_indent("; elif condition");
                        self.gen_expr(elif_cond);
                        self.emit_indent("cmp rax, 0");
                        self.emit_indent(&format!("je {}", next_label));

                        self.emit_indent("; elif block");
                        for stmt in elif_body {
                            self.gen_stmt(stmt);
                        }
                        self.emit_indent(&format!("jmp {}", end_label));
                    }

                    // Else branch (or final label)
                    self.emit(&format!("{}:", next_label));
                    if let Some(else_stmts) = else_body {
                        self.emit_indent("; else block");
                        for stmt in else_stmts {
                            self.gen_stmt(stmt);
                        }
                    }
                }

                self.emit(&format!("{}:", end_label));
                self.emit("");
            }
        }
    }

    fn gen_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Num(n) => {
                self.emit_indent(&format!("mov rax, {}", n));
            }
            Expr::Ident(name) => {
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
                    Op::Eq => {
                        self.emit_indent("cmp rax, rbx");
                        self.emit_indent("sete al");
                        self.emit_indent("movzx rax, al");
                    }
                    Op::NotEq => {
                        self.emit_indent("cmp rax, rbx");
                        self.emit_indent("setne al");
                        self.emit_indent("movzx rax, al");
                    }
                    Op::Gt => {
                        self.emit_indent("cmp rax, rbx");
                        self.emit_indent("setg al");
                        self.emit_indent("movzx rax, al");
                    }
                    Op::Gte => {
                        self.emit_indent("cmp rax, rbx");
                        self.emit_indent("setge al");
                        self.emit_indent("movzx rax, al");
                    }
                    Op::Lt => {
                        self.emit_indent("cmp rax, rbx");
                        self.emit_indent("setl al");
                        self.emit_indent("movzx rax, al");
                    }
                    Op::Lte => {
                        self.emit_indent("cmp rax, rbx");
                        self.emit_indent("setle al");
                        self.emit_indent("movzx rax, al");
                    }
                }
            }
            Expr::UnaryOp(op, expr) => {
                self.gen_expr(expr);
                match op {
                    Op::Sub => {
                        self.emit_indent("neg rax");
                    }
                    _ => {
                        println!("Unary Operator error");
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
mod comparison_tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_comparison_eq() {
        let source = "let x = 5 == 5; exit(x);";
        let tokens = Lexer::new(source).tokenize();
        let stmts = Parser::new(tokens).parse();
        let asm = CodeGen::new().generate(&stmts);

        assert!(asm.contains("cmp rax, rbx"));
        assert!(asm.contains("sete al"));
        assert!(asm.contains("movzx rax, al"));
    }

    #[test]
    fn test_comparison_gt() {
        let source = "let x = 10 > 5; exit(x);";
        let tokens = Lexer::new(source).tokenize();
        let stmts = Parser::new(tokens).parse();
        let asm = CodeGen::new().generate(&stmts);

        assert!(asm.contains("cmp rax, rbx"));
        assert!(asm.contains("setg al"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_simple_exit() {
        let source = "exit(42);";
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
        let source = "let x = 10; exit(x);";
        let tokens = Lexer::new(source).tokenize();
        let stmts = Parser::new(tokens).parse();
        let asm = CodeGen::new().generate(&stmts);

        assert!(asm.contains("mov rax, 10"));
        assert!(asm.contains("mov [rbp-8], rax"));
        assert!(asm.contains("mov rax, [rbp-8]"));
    }

    #[test]
    fn test_arithmetic() {
        let source = "exit(2 + 3 * 4);";
        let tokens = Lexer::new(source).tokenize();
        let stmts = Parser::new(tokens).parse();
        let asm = CodeGen::new().generate(&stmts);

        // Should contain multiplication and addition operations
        assert!(asm.contains("imul rax, rbx"));
        assert!(asm.contains("add rax, rbx"));
    }
}

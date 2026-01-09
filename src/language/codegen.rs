use crate::language::ast::Node;
use crate::asm::*;

pub fn generate(nodes: Vec<Node>) -> String {
    let mut out = String::new();

    for n in nodes {
        let line = match n {
            Node::Mov { dst, src } => mov::emit_mov(&dst, &src),
            Node::Jmp { label } => jmp::emit_jmp(&label),
            Node::Int { num } => int::emit_int(num),
            Node::Ret => ret::emit_ret(),
            Node::Label(name) => format!("{}:", name),
            Node::RawAsm(code) => code.clone(),
            _ => "".to_string(),
        };

        out.push_str(&line);
        out.push('\n');
    }

    out
}

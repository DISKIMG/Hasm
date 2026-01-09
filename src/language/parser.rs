use crate::language::ast::Node;

pub fn parse(input: &str) -> Result<Vec<Node>, String> {
    let mut nodes = Vec::new();

    let mut in_asm_block = false;
    let mut asm_buffer = String::new();

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if line == "asm" {
            in_asm_block = true;
            asm_buffer.clear();
            continue;
        }

        if line == "_asm" {
            in_asm_block = false;
            nodes.push(Node::RawAsm(asm_buffer.clone()));
            asm_buffer.clear();
            continue;
        }

        if in_asm_block {
            asm_buffer.push_str(line);
            asm_buffer.push('\n');
            continue;
        }

        if line.ends_with(':') {
            let name = line.trim_end_matches(':').to_string();
            nodes.push(Node::Label(name));
            continue;
        }

        match parts[0] {
            "set" => nodes.push(Node::Mov {
                dst: parts[1].trim_end_matches(',').to_string(),
                src: parts[2].to_string(),
            }),
            "jump" => nodes.push(Node::Jmp {
                label: parts[1].to_string(),
            }),
            "int" => nodes.push(Node::Int {
                num: parts[1].parse().map_err(|_| "bad int")?,
            }),
            "return" => nodes.push(Node::Ret),
            _ => return Err(format!("Unknown instruction: {}", parts[0])),
        }
    }

    Ok(nodes)
}

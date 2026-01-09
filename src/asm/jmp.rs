pub fn emit_jmp(label: &str) -> String {
    format!("jmp {}", label)
}

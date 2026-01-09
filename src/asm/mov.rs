pub fn emit_mov(dst: &str, src: &str) -> String {
    format!("mov {}, {}", dst, src)
}

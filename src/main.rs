mod asm;
mod language;
mod utils;

use language::parser::parse;
use language::codegen::generate;

fn main() {
    let source = r#"
        main:
            set eax, 1
            jump start
            return

        start:
            set ebx, 2
            asm
            mov eax, 2
            _asm
            jump next

        next:
        asm
            mov eax, 3
            ret
        _asm
    "#;

    let ast = parse(source).expect("parse failed");
    let output = generate(ast);

    println!("{}", output);
}

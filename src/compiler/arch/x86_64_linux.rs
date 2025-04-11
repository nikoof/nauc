use crate::parser::Token;
use indoc::{formatdoc, indoc};

pub fn codegen(program: &[Token], memory: usize) -> String {
    let mut asm: String = formatdoc! {"
        section .bss
            tape        resb {memory}

        section .text
        global _start

        write:
            mov        rdi, 1
            mov        rsi, tape
            add        rsi, r12
            mov        rdx, 1
            mov        rax, 1
            syscall
            ret

        read:
            mov        rdi, 0
            mov        rsi, tape
            add        rsi, r12
            mov        rdx, 1
            mov        rax, 0
            syscall
            ret

        _start:
            mov        r12, 0
    "}
    .to_string();

    for (i, t) in program.iter().enumerate() {
        asm.push_str(&match t {
            Token::Right(operand) => formatdoc! {"
            ;
                add        r12, {operand}
            "}
            .to_string(),
            Token::Left(operand) => formatdoc! {"
            ;
                sub        r12, {operand}
            "}
            .to_string(),
            Token::Add(operand) => formatdoc! {"
            ;
                add        byte [tape + r12], {operand}
            "}
            .to_string(),
            Token::Sub(operand) => formatdoc! {"
            ;
                sub        byte [tape + r12], {operand}
            "}
            .to_string(),
            Token::Read => indoc! {"
            ;
                call       read
            "}
            .to_string(),
            Token::Write => indoc! {"
            ;
                call       write
            "}
            .to_string(),
            Token::Loop(j) => formatdoc! {"
            ;
                movzx      r11, byte [tape + r12]
                cmp        r11, 0
                jne        L{j}
            L{i}:
            "},
            Token::Break(j) => formatdoc! {"
            L{i}:
                movzx      r11, byte [tape + r12]
                cmp        r11, 0
                je         L{j}
            "},
            Token::Comment => "".to_string(),
        });
    }

    asm.push_str(indoc! {"
        exit:
            mov        rax, 60
            mov        rdi, 0
            syscall
    "});

    asm
}

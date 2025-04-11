use crate::parser::Token;
use indoc::formatdoc;

pub fn codegen(program: &[Token], memory: usize) -> String {
    let mut asm = String::new();

    asm.push_str(&formatdoc! {"
        .syntax unified
        .global _start

        .macro mov32, reg, val
            movw \\reg, #:lower16:\\val
            movt \\reg, #:upper16:\\val
        .endm

        .text
        write:
            mov    r0, #1
            add    r1, r5, r4
            mov    r2, #1
            mov    r7, #4
            svc    #0
            mov    pc, lr

        read:
            mov    r0, #0
            add    r1, r5, r4
            mov    r2, #1
            mov    r7, #3
            svc    #0
            mov    pc, lr

        _start:
            mov    r4, #0
            mov32  r5, #tape
    "});

    for (i, token) in program.iter().enumerate() {
        asm.push_str(&match token {
            Token::Right(operand) => {
                formatdoc! {"
                @
                    mov32  r0, #{operand}
                    add    r4, r4, r0
                "}
            }
            Token::Left(operand) => formatdoc! {"
            @
                mov32  r0, #{operand}
                sub    r4, r4, r0
            "},
            Token::Add(operand) => formatdoc! {"
            @
                mov32  r0, #{operand}
                ldrb   r1, [r5, r4]
                add    r1, r0, r1
                strb   r1, [r5, r4]
            "},
            Token::Sub(operand) => formatdoc! {"
            @
                mov32  r0, #{operand}
                ldrb   r1, [r5, r4]
                sub    r1, r1, r0
                strb   r1, [r5, r4]
            "},
            Token::Read => formatdoc! {"
            @
                bl     read
            "},
            Token::Write => formatdoc! {"
            @
                bl     write
            "},
            Token::Loop(j) => formatdoc! {"
                @
                    ldrb   r0, [r5, r4]
                    cmp    r0, #0
                    bne    L{j}
                L{i}:
            "},
            Token::Break(j) => formatdoc! {"
                L{i}:
                    ldrb   r0, [r5, r4]
                    cmp    r0, #0
                    beq    L{j}
            "},
            Token::Comment => String::new(),
        });
    }

    asm.push_str(&formatdoc! {"
        exit:
            mov    r0, #0
            mov    r7, #1
            svc    #0

        .bss
        tape:    .space {memory}, 0x0
    "});

    asm
}

use crate::parser::Token;
use anyhow::{anyhow, Result};
use indoc::{formatdoc, indoc};
use std::{fs::File, io::Write, path::Path, process::Command};

/* TODO: Refactor this.
 * The way these instructions are implemented is obviously unoptimized
 * and far from ideal. */
pub fn codegen(program: &[Token], memory: usize) -> String {
    let mut asm: String = formatdoc! {"
        section .bss
          tape        resb {memory}
          input_buf   resb 10000
          input_index resq 1
          input_len   resq 1

        section .text
        global _start

        read_line:
          mov rax, qword [input_index]
          cmp rax, qword [input_len]
          jne end

          mov qword [input_index], 0
          mov qword [input_len], 0

          read_loop:
            mov rax, 0
            mov rdi, 0,
            mov rcx, qword [input_len]
            lea rsi, byte [input_buf + rcx]
            mov rdx, 1
            syscall

            mov rcx, qword [input_len]
            add qword [input_len], 1
            cmp byte [input_buf + rcx], 10
            jne read_loop

          end:
          ret

        _start:
          mov qword [input_index], 0
          mov qword [input_len], 0
    "}
    .to_string();

    let mut loop_index = 0;
    let mut loops: Vec<usize> = vec![];
    for t in program {
        asm.push_str(&match t {
            Token::Right(count) => formatdoc! {"
                add rbx, {}
            ", count}
            .to_string(),
            Token::Left(count) => formatdoc! {"
                sub rbx, {}
            ", count}
            .to_string(),
            Token::Add(count) => formatdoc! {"
                add byte [tape + rbx], {}
            ", count}
            .to_string(),
            Token::Sub(count) => formatdoc! {"
                sub byte [tape + rbx], {}
            ", count}
            .to_string(),
            Token::Read => indoc! {"
                call read_line
                mov rcx, qword [input_index]
                mov dl, byte [input_buf + rcx]
                mov byte [tape + rbx], dl
                add qword [input_index], 1
            "}.to_string(),
            Token::Write => indoc! {"
                mov rax, 1
                mov rdi, 1
                mov rsi, tape
                add rsi, rbx
                mov rdx, 1
                syscall
            "}
            .to_string(),
            Token::Loop(_) => {
                let loop_index = loops.pop()
                    .expect("Unmatched loops should not get past parser. If you see this, then I messed up the parser.");

                formatdoc! {"
                    movzx rcx, byte [tape + rbx]
                    cmp rcx, 0
                    jne l{loop_index}
                "}
            }
            Token::Break(_) => {
                loops.push(loop_index);
                loop_index += 1;
                formatdoc! {"
                    l{}:
                ", loop_index - 1}
            }
            .to_string(),
            Token::Comment => "".to_string(),
        });
    }

    asm.push_str(indoc! {"
        mov rax, 60
        mov rdi, 0
        syscall
    "});

    asm
}

pub fn compile(asm_source: &String, outfile: &Path, keep_artifacts: bool) -> Result<()> {
    let asm = outfile.with_extension("asm");
    let obj = outfile.with_extension("o");
    let bin = outfile
        .file_stem()
        .ok_or(anyhow!("Output file is a directory"))?;

    File::create(&asm)?.write_all(asm_source.as_bytes())?;

    let nasm = Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg("-o")
        .arg(&obj)
        .arg(&asm)
        .output()?;
    eprintln!("{}", String::from_utf8_lossy(&nasm.stderr));

    let link = Command::new("ld").arg("-o").arg(bin).arg(&obj).output()?;
    eprintln!("{}", String::from_utf8_lossy(&link.stderr));

    if !keep_artifacts {
        std::fs::remove_file(&asm)?;
        std::fs::remove_file(&obj)?;
    }

    Ok(())
}

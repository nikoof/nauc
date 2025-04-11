use anyhow::{anyhow, Result};
use std::{fs::File, io::Write, path::Path, process::Command};

pub mod arch;

pub fn compile(
    asm_source: &String,
    outfile: &Path,
    keep_artifacts: bool,
    debug: bool,
) -> Result<()> {
    let asm = outfile.with_extension("s");
    let obj = outfile.with_extension("o");
    let bin = outfile
        .file_stem()
        .ok_or(anyhow!("Output file is a directory"))?;

    File::create(&asm)?.write_all(asm_source.as_bytes())?;

    // let nasm = Command::new("nasm")
    //     .args(["-f", "elf64"])
    //     .args(if debug {
    //         vec!["-g", "-F", "dwarf"]
    //     } else {
    //         vec![]
    //     })
    //     .arg("-o")
    //     .arg(&obj)
    //     .arg(&asm)
    //     .output()?;

    let nasm = Command::new("arm-none-eabi-as")
        .args(if debug { vec!["-g"] } else { vec![] })
        .arg("-o")
        .arg(&obj)
        .arg(&asm)
        .output()?;

    eprintln!("{}", String::from_utf8_lossy(&nasm.stderr));

    let link = Command::new("arm-none-eabi-ld")
        .arg("-o")
        .arg(bin)
        .arg(&obj)
        .output()?;
    eprintln!("{}", String::from_utf8_lossy(&link.stderr));

    if !keep_artifacts {
        std::fs::remove_file(&asm)?;
        std::fs::remove_file(&obj)?;
    }

    Ok(())
}

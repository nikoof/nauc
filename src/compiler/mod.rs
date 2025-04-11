use anyhow::{anyhow, bail, Result};
use std::{fs::File, io::Write, path::Path, process::Command};

use self::arch::Target;

pub mod arch;

pub fn compile(
    asm_source: &String,
    outfile: &Path,
    keep_artifacts: bool,
    debug: bool,
    target: Target,
) -> Result<()> {
    let asm = outfile.with_extension("s");
    let obj = outfile.with_extension("o");
    let bin = outfile
        .file_stem()
        .ok_or(anyhow!("Output file is a directory"))?;

    File::create(&asm)?.write_all(asm_source.as_bytes())?;

    // TODO: add local platform detection and set default accordingly
    // TODO: find a way to use the platform's native assembler

    let assembler = match target {
        Target::Aarch32Linux => Command::new("arm-none-gnuabi-as")
            .args(if debug { vec!["-g"] } else { vec![] })
            .arg("-o")
            .arg(&obj)
            .arg(&asm)
            .output(),
        Target::X86_64Linux => Command::new("nasm")
            .args(["-f", "elf64"])
            .args(if debug {
                vec!["-g", "-F", "dwarf"]
            } else {
                vec![]
            })
            .arg("-o")
            .arg(&obj)
            .arg(&asm)
            .output(),
    }?;

    if !assembler.status.success() {
        if !keep_artifacts {
            if asm.exists() {
                std::fs::remove_file(&asm)?;
            }
            if obj.exists() {
                std::fs::remove_file(&obj)?;
            }
        }

        bail!(
            "Compilation error:\n{}",
            String::from_utf8_lossy(&assembler.stderr)
        );
    }

    let linker = match target {
        Target::Aarch32Linux => Command::new("arm-none-gnuabi-ld")
            .arg("-o")
            .arg(bin)
            .arg(&obj)
            .output(),
        Target::X86_64Linux => Command::new("ld").arg("-o").arg(bin).arg(&obj).output(),
    }?;

    if !linker.status.success() {
        if !keep_artifacts {
            if asm.exists() {
                std::fs::remove_file(&asm)?;
            }
            if obj.exists() {
                std::fs::remove_file(&obj)?;
            }
            if Path::new(bin).exists() {
                std::fs::remove_file(bin)?;
            }
        }

        bail!(
            "Linking error:\n{}",
            String::from_utf8_lossy(&linker.stderr)
        );
    }

    if !keep_artifacts {
        if asm.exists() {
            std::fs::remove_file(&asm)?;
        }
        if obj.exists() {
            std::fs::remove_file(&obj)?;
        }
    }

    Ok(())
}

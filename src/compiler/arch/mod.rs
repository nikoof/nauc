use anyhow::anyhow;
use clap::{builder::PossibleValue, ValueEnum};

pub mod aarch64_linux;
pub mod x86_64_linux;

#[derive(Debug, Clone, Copy)]
pub enum Architecture {
    Aarch64Linux,
    X86_64Linux,
}

impl ValueEnum for Architecture {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Aarch64Linux, Self::X86_64Linux]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(Into::<&str>::into(*self)))
    }
}

impl From<Architecture> for &str {
    fn from(value: Architecture) -> Self {
        match value {
            Architecture::Aarch64Linux => "aarch64-linux",
            Architecture::X86_64Linux => "x86_64-linux",
        }
    }
}

impl TryFrom<&str> for Architecture {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "aarch64-linux" => Ok(Architecture::Aarch64Linux),
            "x86_64-linux" => Ok(Architecture::X86_64Linux),
            _ => Err(anyhow!(
                "{} is not a valid architecture or it is not implemented",
                &value
            )),
        }
    }
}

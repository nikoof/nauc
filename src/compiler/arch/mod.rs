use anyhow::anyhow;
use clap::{builder::PossibleValue, ValueEnum};

pub mod aarch32_linux;
pub mod x86_64_linux;

#[derive(Debug, Clone, Copy)]
pub enum Target {
    Aarch32Linux,
    X86_64Linux,
}

impl ValueEnum for Target {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Aarch32Linux, Self::X86_64Linux]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(Into::<&str>::into(*self)))
    }
}

impl From<Target> for &str {
    fn from(value: Target) -> Self {
        match value {
            Target::Aarch32Linux => "aarch32-linux",
            Target::X86_64Linux => "x86_64-linux",
        }
    }
}

impl TryFrom<&str> for Target {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "aarch64-linux" => Ok(Target::Aarch32Linux),
            "x86_64-linux" => Ok(Target::X86_64Linux),
            _ => Err(anyhow!(
                "{} is not a valid architecture or it is not implemented",
                &value
            )),
        }
    }
}

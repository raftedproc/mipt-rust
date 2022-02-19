use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Command {
    ForbidUnsafe,
    CargoFmt,
    CargoClippy,
    CargoTest,
}

impl Command {
    pub fn from_name(name: &str) -> Result<Self> {
        Ok(match name {
            "forbid-unsafe" => Self::ForbidUnsafe,
            "cargo-fmt" => Self::CargoFmt,
            "cargo-clippy" => Self::CargoClippy,
            "cargo-test" => Self::CargoTest,
            name => bail!("command \"{name}\" is not supported"),
        })
    }

    pub fn get_shell_line(&self) -> Result<String> {
        Ok(match self {
            Self::ForbidUnsafe => bail!("no shell line for ForbidUnsafe"),
            Self::CargoFmt => "cargo fmt --check".to_string(),
            Self::CargoClippy => "cargo clippy --release -- -D warnings".to_string(),
            Self::CargoTest => "cargo test --release".to_string(),
        })
    }
}

use anyhow::{bail, Result};

#[derive(Debug)]
pub enum Command {
    ForbidUnsafe,
    CargoFmt,
    CargoClippy,
    CargoTest,
    CargoCompileTestMiniFrunk,
    CargoCompileTestOrm,
    PythonTest,
    ForbidCollections,
}

impl Command {
    pub fn from_name(name: &str) -> Result<Self> {
        Ok(match name {
            "forbid-unsafe" => Self::ForbidUnsafe,
            "cargo-fmt" => Self::CargoFmt,
            "cargo-clippy" => Self::CargoClippy,
            "cargo-test" => Self::CargoTest,
            "python-test" => Self::PythonTest,
            "forbid-collections" => Self::ForbidCollections,
            "cargo-compile-test-mini-frunk" => Self::CargoCompileTestMiniFrunk,
            "cargo-compile-test-orm" => Self::CargoCompileTestOrm,
            name => bail!("command \"{name}\" is not supported"),
        })
    }

    pub fn get_shell_line(&self) -> Result<String> {
        Ok(match self {
            Self::ForbidUnsafe => bail!("no shell line for ForbidUnsafe"),
            Self::ForbidCollections => bail!("no shell line for ForbidCollections"),
            Self::CargoCompileTestMiniFrunk => bail!("no shell line for CargoCompileTestMiniFrunk"),
            Self::CargoCompileTestOrm => bail!("no shell line for CargoCompileTestOrm"),
            Self::CargoFmt => "cargo fmt --check".to_string(),
            Self::CargoClippy => "cargo clippy --release -- -D warnings".to_string(),
            Self::CargoTest => "cargo test --release".to_string(),
            Self::PythonTest => "python3 test.py".to_string(),
        })
    }
}

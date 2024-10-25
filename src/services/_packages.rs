use async_process::Command;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Package {
    id: u8,
    name: String,
}

impl Package {
    pub fn new(id: u8, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub async fn is_installed(&self) -> bool {
        Command::new(self.get_name())
            .arg("--version")
            .output().await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

pub enum Packages {
    PNPM,
    YARN,
    NPM,
    BUN,
}

impl Packages {
    pub fn find_by_name(name: &str) -> Self {
        match name {
            "bun" => Self::BUN,
            "pnpm" => Self::PNPM,
            "yarn" => Self::YARN,
            _ => Self::NPM,
        }
    }

    pub fn find_by_id(id: u8) -> Self {
        match id {
            1 => Self::BUN,
            2 => Self::PNPM,
            3 => Self::YARN,
            4 => Self::NPM,
            _ => Self::BUN,
        }
    }

    fn default(&self) -> Package {
        match self {
            Self::BUN => Package::new(1, "bun"),
            Self::PNPM => Package::new(2, "pnpm"),
            Self::YARN => Package::new(3, "yarn"),
            Self::NPM => Package::new(4, "npm"),
        }
    }

    pub fn get(&self) -> Package {
        self.default()
    }

    pub fn get_name(&self) -> String {
        self.get().name
    }

    pub fn get_id(&self) -> u8 {
        self.get().id
    }
}

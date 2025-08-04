use std::path::{Path, PathBuf};

use async_fs::File;
use futures_lite::AsyncReadExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FeatureMapping {
    pub(crate) route: String,
    pub(crate) model: String,
    pub(crate) controller: String,
    pub(crate) file: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mapping {
    features: Vec<FeatureMapping>,
}

pub struct UranMapping {
    mapping: Mapping,
    path: PathBuf,
}
impl UranMapping {
    pub async fn load_routes<P>(path: P) -> Result<Self, Box<dyn std::error::Error>> 
    where
        P: AsRef<Path> + Into<PathBuf>,
    {
        let mut file = File::open(&path).await?;
        let mut data = String::new();
        file.read_to_string(&mut data).await?;
        let mapping: Mapping = serde_json::from_str(&data)?;
        Ok(Self { mapping, path: path.into() })
    }

    pub fn has_route(&self, route: &str) -> bool {
        self.mapping.features.iter().any(|r| r.route == route)
    }
    pub fn has_controller(&self, controller: &str) -> bool {
        self.mapping.features.iter().any(|r| r.controller == controller)
    }
    pub fn has_model(&self, model: &str) -> bool {
        self.mapping.features.iter().any(|r| r.model == model)
    }
    pub fn has_file(&self, file: &str) -> bool {
        self.mapping.features.iter().any(|r| r.file == file)
    }

    pub async fn add_feature(&mut self, feature: FeatureMapping) {
        self.mapping.features.push(feature);
    }

    async fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(&self.mapping)?;
        async_fs::write(&self.path, data).await?;
        Ok(())
    }
}
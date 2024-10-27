use std::{fs::File, io::Write, path::{Path, PathBuf}};

use actix_rt::Arbiter;
use async_process::Command;
use sailfish::TemplateSimple;
use serde::Serialize;
use template::Template;

mod template {
    use sailfish::TemplateSimple;

    pub trait Template {
        fn filename(&self) -> String;
    }

    #[derive(TemplateSimple, Clone)]
    #[template(path = "env.stpl")]
    pub struct EnvTemplate {
        pub name: String,
    }
    impl Template for EnvTemplate {
        fn filename(&self) -> String {
            ".env".to_string()
        }
    }

    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/index.stpl")]
    pub struct IndexTemplate;
    impl Template for IndexTemplate {
        fn filename(&self) -> String {
            "index.js".to_string()
        }
    }

    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/models/index.stpl")]
    pub struct IndexModelsTemplate;
    impl Template for IndexModelsTemplate {
        fn filename(&self) -> String {
            "index.js".to_string()
        }
    }

    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/models/base.stpl")]
    pub struct BaseModelsTemplate;
    impl Template for BaseModelsTemplate {
        fn filename(&self) -> String {
            "base.js".to_string()
        }
    }

    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/controller/index.stpl")]
    pub struct IndexControllersTemplate;
    impl Template for IndexControllersTemplate {
        fn filename(&self) -> String {
            "index.js".to_string()
        }
    }

    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/controller/base.stpl")]
    pub struct BaseControllerTemplate;
    impl Template for BaseControllerTemplate {
        fn filename(&self) -> String {
            "base.js".to_string()
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Package {
    id: u8,
    name: String,
    y: bool,
}

impl Package {
    pub fn new(id: u8, name: &str, y: bool) -> Self {
        Self {
            id,
            name: name.to_string(),
            y,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub async fn is_installed(&self) -> bool {
        Command::new(self.get_name())
            .arg("--version")
            .output()
            .await
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    async fn create_file<T: TemplateSimple + Template + Clone + Send + 'static>(&self, ctx: T, dir: PathBuf) {
        let arbiter = Arbiter::new();
        arbiter.spawn(async move {
            let dir = dir.clone();
            if !dir.exists() {
                std::fs::create_dir_all(&dir).unwrap();
            }
            let ctx = ctx.clone();
            let mut file = File::create(dir.join(ctx.filename().as_str())).unwrap();
            let render = ctx.render_once().unwrap();
            file.write_all(render.as_bytes()).unwrap();
        });
    }

    pub async fn init(&self, dir: &Path) -> bool {
        let mut file = File::create(dir.join("package.json")).unwrap();

        let data = format!(
            r#"{{
  "name": "{}",
  "type": "module",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {{
      "dev": "nodemon src/index.js"
  }},
  "keywords": [],
  "author": "",
  "license": "ISC",
  "dependencies": {{
    "@prisma/client": "^5.21.1",
    "bcrypt": "^5.1.1",
    "cookie-parser": "^1.4.7",
    "cors": "^2.8.5",
    "date-fns": "^4.1.0",
    "express": "^4.21.1",
    "jsonwebtoken": "^9.0.2",
    "uuid": "^10.0.0",
    "zod": "^3.23.8"
  }},
  "devDependencies": {{
    "nodemon": "^2.0.15",
    "prisma": "^5.21.1"
  }}
}}"#,
            dir.file_name().unwrap().to_str().unwrap()
        );
        file.write_all(data.as_bytes()).unwrap();

        let arbiter = Arbiter::new();
        let pkg_name = self.get_name().to_string();
        let project_dir = dir.to_path_buf().clone();
        arbiter.spawn(async move {
            Command::new(&pkg_name)
                .current_dir(project_dir)
                .arg("install")
                .output()
                .await
                .map(|output| output.status.success())
                .unwrap_or(false);
        });
        
        let ctx = template::EnvTemplate { name: "token".to_string() };
        self.create_file(ctx, dir.to_path_buf().clone()).await;

        let ctx = template::IndexTemplate;
        self.create_file(ctx, dir.to_path_buf().join("src").clone()).await;

        let ctx = template::IndexControllersTemplate;
        self.create_file(ctx, dir.to_path_buf().join("src").join("controller").clone()).await;

        let ctx = template::IndexModelsTemplate;
        self.create_file(ctx, dir.to_path_buf().join("src").join("models").clone()).await;

        let ctx = template::BaseControllerTemplate;
        self.create_file(ctx, dir.to_path_buf().join("src").join("controller").clone()).await;

        let ctx = template::BaseModelsTemplate;
        self.create_file(ctx, dir.to_path_buf().join("src").join("models").clone()).await;

        let mut file = File::create(dir.join("api_config.json")).unwrap();

        let data = r#"{{
  "name": "{}",
  "type": "module",
}}"#;
        file.write_all(data.as_bytes()).unwrap();

        let dir_routes = dir.join("src").join("routes");
        if !dir_routes.exists() {
            std::fs::create_dir_all(&dir_routes).unwrap();
        }
        File::create(dir_routes.join("index.js")).unwrap();

        return false;
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
            Self::BUN => Package::new(1, "bun", true),
            Self::PNPM => Package::new(2, "pnpm", false),
            Self::YARN => Package::new(3, "yarn", true),
            Self::NPM => Package::new(4, "npm", true),
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

use std::{fs::File, io::Write, path::{Path, PathBuf}};

use actix_rt::Arbiter;
use async_process::Command;
use sailfish::TemplateSimple;
use serde::Serialize;
use template::*;

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
    #[template(path = "js/controllers/index.stpl")]
    pub struct IndexControllersTemplate;
    impl Template for IndexControllersTemplate {
        fn filename(&self) -> String {
            "index.js".to_string()
        }
    }

    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/controllers/base.stpl")]
    pub struct BaseControllerTemplate;
    impl Template for BaseControllerTemplate {
        fn filename(&self) -> String {
            "base.js".to_string()
        }
    }
    
    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/configs/_allowedOrigins.stpl")]
    pub struct AllowedConfigTemplate;
    impl Template for AllowedConfigTemplate {
        fn filename(&self) -> String {
            "_allowedOrigins.js".to_string()
        }
    }

    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/configs/_corsOptions.stpl")]
    pub struct CorsConfigTemplate;
    impl Template for CorsConfigTemplate {
        fn filename(&self) -> String {
            "_corsOptions.js".to_string()
        }
    }
    
    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/configs/index.stpl")]
    pub struct IndexConfigTemplate;
    impl Template for IndexConfigTemplate {
        fn filename(&self) -> String {
            "index.js".to_string()
        }
    }
    
    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/database/_prisma.stpl")]
    pub struct PrismaDatabaseTemplate;
    impl Template for PrismaDatabaseTemplate {
        fn filename(&self) -> String {
            "_prisma.js".to_string()
        }
    }
    
    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/database/index.stpl")]
    pub struct IndexDatabaseTemplate;
    impl Template for IndexDatabaseTemplate {
        fn filename(&self) -> String {
            "index.js".to_string()
        }
    }
    
    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/middleware/_credentials.stpl")]
    pub struct CredentialsMiddlewareTemplate;
    impl Template for CredentialsMiddlewareTemplate {
        fn filename(&self) -> String {
            "_credentials.js".to_string()
        }
    }
    
    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/middleware/index.stpl")]
    pub struct IndexMiddlewareTemplate;
    impl Template for IndexMiddlewareTemplate {
        fn filename(&self) -> String {
            "index.js".to_string()
        }
    }
    
    #[derive(TemplateSimple, Clone)]
    #[template(path = "js/routes/index.stpl")]
    pub struct IndexRoutesTemplate;
    impl Template for IndexRoutesTemplate {
        fn filename(&self) -> String {
            "index.js".to_string()
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

    fn create_file<T: TemplateSimple + Template + Clone + Send + 'static>(&self, ctx: T, dir: PathBuf) {
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
    
    fn create_architecture(&self, src: PathBuf) {
        let controller = src.join("controllers");
        let models = src.join("models");
        let database = src.join("database");
        let middleware = src.join("middleware");
        let configs = src.join("configs");
        let routes = src.join("routes");
        
        self.create_file(IndexTemplate, src.clone());
        self.create_file(IndexControllersTemplate, controller.clone());
        self.create_file(IndexModelsTemplate, models.clone());
        self.create_file(IndexDatabaseTemplate, database.clone());
        self.create_file(IndexMiddlewareTemplate, middleware.clone());
        self.create_file(IndexConfigTemplate, configs.clone());
        self.create_file(BaseControllerTemplate, controller.clone());
        self.create_file(BaseModelsTemplate, models.clone());
        self.create_file(PrismaDatabaseTemplate, database.clone());
        self.create_file(CredentialsMiddlewareTemplate, middleware.clone());
        self.create_file(AllowedConfigTemplate, configs.clone());
        self.create_file(CorsConfigTemplate, configs.clone());
        self.create_file(IndexRoutesTemplate, routes.clone());
    }

    pub async fn init(&self, dir: &Path) -> bool {
        let mut file = File::create(dir.join("package.json")).unwrap();

        let data = format!(
            r#"{{
  "name": "{}",
  "version": "1.0.0",
  "description": "",
  "main": "src/index.js",
  "scripts": {{
      "dev": "nodemon --exec babel-node ."
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
    "babel-plugin-module-resolver": "^5.0.2",
    "@babel/cli": "^7.18.10",
    "@babel/core": "^7.22.11",
    "@babel/node": "^7.22.10",
    "@babel/plugin-transform-object-rest-spread": "^7.24.7",
    "@babel/preset-env": "^7.22.10",
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
        self.create_file(ctx, dir.to_path_buf().clone());

        let src = dir.to_path_buf().join("src");
        self.create_architecture(src.clone());

        let mut file = File::create(dir.join("uran.toml")).unwrap();

        let data = format!(r#"[config]
project = "{}"
description = ""
author = ""
package = "{}"
language = "js"
framework = "express"
version = "1.0.0"

[lines]
route = "5"

"#, dir.file_name().unwrap().to_str().unwrap(), self.get_name());
        file.write_all(data.as_bytes()).unwrap();
        
        let mut file = File::create(dir.join(".babelrc")).unwrap();
        let data = format!(r#"{{
"presets": ["@babel/preset-env"],
"plugins": [
    "@babel/plugin-transform-object-rest-spread",
    [ "module-resolver", {{ "alias" : {{ "~": "./src"}} }} ] 
  ]
}}
"#);
        file.write_all(data.as_bytes()).unwrap();
        
        let mut file = File::create(dir.join("jsconfig.json ")).unwrap();
        let data = format!(r#"{{
  "compilerOptions": {{
    "baseUrl": ".",
    "paths": {{
      "~/*": [ "./src/*" ],
    }}
  }}
}}
"#);
        file.write_all(data.as_bytes()).unwrap();

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
    // pub fn find_by_name(name: &str) -> Self {
    //     match name {
    //         "bun" => Self::BUN,
    //         "pnpm" => Self::PNPM,
    //         "yarn" => Self::YARN,
    //         _ => Self::NPM,
    //     }
    // }

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
}

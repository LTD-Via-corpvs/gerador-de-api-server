use std::{fs, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub project: String,
    pub description: String,
    pub author: String,
    pub package: String,
    pub language: String,
    pub framework: String,
    pub version: String
}

#[derive(Deserialize, Serialize)]
pub struct Lines {
    pub route: String
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub config: Config,
    pub lines: Lines,
}

pub struct Project {
    pub data: Data,
}
impl Project {
    pub fn read(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).ok();
        let data = toml::from_str::<Data>(&contents.unwrap()).unwrap();
        Self { data }
    }
    
    pub fn generate_model(&self, model_name: &str, file_name: &str, junction_table: &str) -> std::io::Result<()> {
        let model_code = format!(
            "import {{ BaseModel }} from './index.js';\n\n\
            const {0}Model = () => {{\n\
            const base = BaseModel(\n\t\t{{\n\
                \tmodel: '{1}',\n\
                \tjunctionTable: '{2}'\n\
            \t}})\n\
            return {{\n\
                ...base\n\
            }}\n\
            }}\n\n\
            export default {{ {0}Model }}\n\
            export {{ {0}Model }}\n",
            model_name, model_name.to_lowercase(), junction_table
        );

        let project_path = &self.get_path();
        let file_path = project_path
            .join("src")
            .join("models")
            .join(format!("_{}.js", file_name));

        let index_path = project_path
            .join("src")
            .join("models")
            .join("index.js");

        fs::write(&file_path, model_code)?;

        let export_line = format!("\nexport * from './_{}.js'", file_name);
        fs::OpenOptions::new()
            .append(true)
            .open(index_path)?
            .write_all(export_line.as_bytes())?;

        Ok(())
    }
    
    pub fn generate_controller(&self, model_name: &str, file_name: &str) -> std::io::Result<()> {
        let controller_code = format!(
            "import {{ {0}Model }} from '../models/index.js'\n\
            import {{ BaseController }} from './index.js'\n\
            const {{save, getOne, getAll, update, remove, getTotalObjects}} = {0}Model()\n\n\
            const {0}Controller = () => {{\n\
                const include = {{}}\n\
                const base = BaseController(\n\
                    {{\n\
                        save: save,\n\
                        getOne: getOne,\n\
                        getAll: getAll,\n\
                        update: update,\n\
                        remove: remove,\n\
                        getTotalObjects: getTotalObjects,\n\
                        include: include,\n\
                    }}\n\
                )\n\
                return {{\n\
                    ...base,\n\
                }}\n\
            }}\n\n\
            export default {0}Controller\n\
            export {{ {0}Controller }}\n",
            model_name
        );

        let project_path = &self.get_path();
        let file_path = project_path
            .join("src")
            .join("controllers")
            .join(format!("_{}.js", file_name));

        let index_path = project_path
            .join("src")
            .join("controllers")
            .join("index.js");

        fs::write(&file_path, controller_code)?;

        let export_line = format!("\nexport * from './_{}.js'", file_name);
        fs::OpenOptions::new()
            .append(true)
            .open(index_path)?
            .write_all(export_line.as_bytes())?;

        Ok(())
    }
    
    pub fn generate_route(&self, model_name: &str, file_name: &str, route_name: &str) -> std::io::Result<()> {
        let router_code = format!(
            "import {{ {0}Controller }} from '../controllers/index.js'\n\
            import {{ Router }} from 'express'\n\
            const router = Router()\n\n\
            const {{ create, readOne, readAll, updateObj, removeObj }} = {0}Controller()\n\n\
            router.route('/')\n\
                .post(create)\n\
                .get(readAll)\n\n\
            router.route('/:id')\n\
                .get(readOne)\n\
                .put(updateObj)\n\
                .delete(removeObj)\n\n\
            export default router;\n\
            export {{ router as {0}Routes }};\n",
            model_name
        );

        let project_path = &self.get_path();
        let file_path = project_path
            .join("src")
            .join("routes")
            .join(format!("_{}.js", file_name));

        let index_path = project_path
            .join("src")
            .join("routes")
            .join("index.js");

        fs::write(&file_path, router_code)?;

        // Add import line to the routes index.js file at the beginning
        let import_line = format!("import {{ {0}Routes }} from './_{1}.js';\n", model_name, file_name);
        let current_content = fs::read_to_string(&index_path)?;
        let new_content = format!("{}{}", import_line, current_content);
        fs::write(index_path, new_content)?;

        // Update the main routes file to use the new routes
        let main_routes_path = project_path.join("src").join("routes").join("index.js");
        let route_content = fs::read_to_string(&main_routes_path)?;
        let route_lines: Vec<&str> = route_content.lines().collect();

        let route_line_number: usize = self.data.lines.route.parse().unwrap_or(5);
        let new_route_line = format!("routes.use('/{0}', {1}Routes);", route_name.to_lowercase(), model_name);

        let mut updated_content = String::new();
        for (i, line) in route_lines.iter().enumerate() {
            if i == route_line_number {
                updated_content.push_str(line);
                updated_content.push_str("\n");
                updated_content.push_str(&new_route_line);
                updated_content.push_str("\n");
            } else {
                updated_content.push_str(line);
                updated_content.push_str("\n");
            }
        }

        fs::write(main_routes_path, updated_content)?;

        Ok(())
    }
    
    fn get_path(&self) -> std::path::PathBuf {
        let project_name = &self.data.config.project;
        let build = std::path::Path::new("../build");
        build.join(&project_name)
    }
}
from os import path, getcwd, mkdir

class TemplateManager:
    def __init__(self, script_folder) -> None:
        print(script_folder)
        self.build_path = path.join(script_folder, '..', '..', 'build')

    def update_project_path(self, project_name):
        self.project_path = path.join(self.build_path, project_name)
    
    def create_index(self):
        content = (
            "import express from \"express\";\n"
            "const app = express();\n"
            "const PORT = process.env.PORT;\n"
            """
app.listen(PORT, () => {
  console.log(`Server Running on PORT: ${PORT}!`);
})
"""
        )
        if not path.exists(path.join(self.project_path, "src")):
            mkdir(path.join(self.project_path, "src"))
        if not self.__create_file(path.join("src","index.js"), content):
            raise Exception("src/index.js already exists")

    def create_env(self):
        self.__create_env_example()
        content = (
            "DATABASE_URL=\"\""
            "REFRESH_TOKEN=\"\""
            "ACCESS_TOKEN=\"\""
            "PORT=8080"
        )
        if not self.__create_file(".env", content):
            raise Exception(".env already exists")
        
    def __create_env_example(self):
        content = (
            "DATABASE_URL=\"mysql://root:root@localhost:3306/api-generato\""
            "REFRESH_TOKEN=\"refresh_token\""
            "ACCESS_TOKEN=\"access_token\""
            "PORT=your_port"
        )
        if not self.__create_file(".env.example", content):
            raise Exception(".env.example already exists")

    def __create_file(self, file_name, content) -> bool:
        path_file = path.join(self.project_path, file_name)
        if path.exists(path_file):
            return False
        with open(path_file, 'w', encoding='utf-8') as file:
            file.write(content)
        return True
from os import path, listdir, mkdir
import pathlib
from shutil import copy2, copytree

class TemplateManager:
    def __init__(self, script_folder) -> None:
        self.build_path = path.join(script_folder, '..', '..', 'build')

    def update_project_path(self, project_name):
        self.project_path = path.join(self.build_path, project_name)
    
    def create_index(self):
        content = (
            "import express from \"express\"\n"
            "import cookieParser from \"cookie-parser\"\n"
            "import cors from \"cors\"\n"
            "const PORT = 3333;\n"
            "\n"
            "import { credentials } from \"./middleware/index.js\"\n"
            "import { corsOptions } from \"./configs/index.js\"\n"
            "\n"
            "//Initializing express\n"
            "const app = express();\n"
            "\n"
            "//Credentials check before CORS!\n"
            "app.use(credentials);\n"
            "\n"
            "//Solve CORS\n"
            "app.use(cors(corsOptions));\n"
            "\n"
            "//Config JSON response\n"
            "app.use(express.json());\n"
            "\n"
            "//Middleware for cookies\n"
            "app.use(cookieParser());\n"
            "\n"
            "//Routes\n"
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
        
    def create_src(self):
        if not self.__create_file("src"):
            raise Exception("src already exists")

    def __create_file(self, file_name, content = None) -> bool:
        path_file = path.join(self.project_path, file_name)
        if path.exists(path_file):
            return False
        if content is not None:
            with open(path_file, 'w', encoding='utf-8') as file:
                file.write(content)
        else:
            self.__copytree(path.join(pathlib.Path(__file__).parent.resolve(), "..", "template", file_name), path_file)
        return True

    def __copytree(self, src: str, dst: str, symlinks: bool = False, ignore=None):
        if path.isdir(src):
            for item in listdir(src):
                s = path.join(src, item)
                d = path.join(dst, item)
                if path.isdir(s):
                    copytree(s, d, symlinks, ignore)
                else:
                    copy2(s, d)
        else:
            copy2(src, dst)
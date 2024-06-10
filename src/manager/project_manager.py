from os import path, listdir, walk
import re

class ProjectManager:
    def __init__(self, script_folder) -> None:
        self.build_path = path.join(script_folder, '..', '..', 'build')

    def get_projects(self):
        return next(walk(self.build_path))[1]

    def update_project_path(self, project_name):
        self.project_path = path.join(self.build_path, project_name)
        if not path.exists(self.project_path):
            raise NotADirectoryError(f"O projeto {project_name} não existe.")

    def generateModel(self, modelName: str, fileName: str, junctionTable: str):
        modelCode = (
            f"import {{ BaseModel }} from './index.js';\n\n"
            f"const {modelName}Model = () => {{\n"
            f"    const base = BaseModel(\n\t\t{{\n"
            f"        \tmodel: '{modelName.lower()}',\n"
            f"        \tjunctionTable: '{junctionTable}'\n"
            f"    \t}})\n"
            f"    return {{\n"
            f"        ...base\n"
            f"    }}\n"
            f"}}\n\n"
            f"export default {{ {modelName}Model }}\n"
            f"export {{ {modelName}Model }}\n"
        )
        filePath = path.join(self.project_path, "src", "models", f"_{fileName}.js")
        indexPath = path.join(self.project_path, "src", "models", "index.js")
        with open(filePath, 'w') as f:
            f.write(modelCode)
        with open(indexPath, 'a') as f:
            f.write(f"\nexport * from './_{fileName}.js'")

    def generateController(self, modelName: str, fileName: str):
        controller_code = (
            f"import {{ {modelName}Model }} from '../models/index.js'\n"
            f"import {{ BaseController }} from './index.js'\n"
            f"const {{save, getOne, getAll, update, remove, getTotalObjects}} = {modelName}Model()\n\n"
            f"const {modelName}Controller = () => {{\n"
            f"    const include = {{}}\n"
            f"    const base = BaseController(\n"
            f"        {{\n"
            f"            save: save,\n"
            f"            getOne: getOne,\n"
            f"            getAll: getAll,\n"
            f"            update: update,\n"
            f"            remove: remove,\n"
            f"            getTotalObjects: getTotalObjects,\n"
            f"            include: include,\n"
            f"        }}\n"
            f"    )\n"
            f"    return {{\n"
            f"        ...base,\n"
            f"    }}\n"
            f"}}\n\n"
            f"export default {modelName}Controller\n"
            f"export {{ {modelName}Controller }}\n"
        )
        filePath = path.join(self.project_path, "src", "controllers", f"_{fileName}.js")
        indexPath = path.join(self.project_path, "src", "controllers", "index.js")
        with open(filePath, 'w') as f:
            f.write(controller_code)
        with open(indexPath, 'a') as f:
            f.write(f"\nexport * from './_{fileName}.js'")

    def generateRoute(self, modelName, fileName):
        router_code = (
            f"import {{ {modelName}Controller }} from '../controllers/index.js'\n"
            f"import {{ Router }} from 'express'\n"
            f"const router = Router()\n\n"
            f"const {{ create, readOne, readAll, updateObj, removeObj }} = {modelName}Controller()\n\n"
            f"router.route('/')\n"
            f"    .post(create)\n"
            f"    .get(readAll)\n\n"
            f"router.route('/:id')\n"
            f"    .get(readOne)\n"
            f"    .put(updateObj)\n"
            f"    .delete(removeObj)\n\n"
            f"export default router;\n"
            f"export {{ router as {modelName}Routes }};\n"
        )
        filePath = path.join(self.project_path, "src", "routes", f"_{fileName}.js")
        indexPath = path.join(self.project_path, "src", "routes", "index.js")
        with open(filePath, 'w') as f:
            f.write(router_code)
        with open(indexPath, 'a') as f:
            f.write(f"\nexport * from './_{fileName}.js'")

    def insertRouteIntoIndex(self, routeName, modelName):
        new_code = f"app.use('/{routeName}', {modelName}Routes);\n"
        indexPath = path.join(self.project_path, "src", "index.js")
        with open(indexPath, "r+") as f:
            lines = f.readlines()
            for i, line in enumerate(lines):
                if "//Routes" in line:
                    lines.insert(i + 1, new_code)
                    f.seek(0)
                    f.writelines(lines)
                    break

    def insertImportIntoIndex(self, modelName):
        palavra_adicionar = f'{modelName}Routes'
        indexPath = path.join(self.project_path, "src", "index.js")

        with open(indexPath, 'r') as arquivo:
            linhas = arquivo.readlines()

        linha_desejada = None
        index = 0
        abc = []
        for i, linha in enumerate(linhas[:9]):
            splited = linha.split(" ")
            if '"./routes/index.js"\n' in splited:
                linha_desejada = linhas[i]
                index = i
                break
            abc.append(splited)
        # raise Exception(abc)
        if linha_desejada is None:
            index = 7
            nova_linha = 'import {} from "./routes/index.js"\n\n'.format('{ '+palavra_adicionar+' }')
        else:
            result = re.search('{(.*)}', linha_desejada)
            splited = result.group(1).strip().split(", ")
            # raise Exception(splited)
            if palavra_adicionar in splited:
                raise Exception("This route has already been defined")
            nova_linha = linha_desejada.replace('}', f', {palavra_adicionar}' + ' }')

        linhas[index] = nova_linha

        with open(indexPath, 'w') as arquivo:
            arquivo.writelines(linhas)

    def getModels(self):
        dir_path = path.join(self.project_path, "src", "models")
        files = [f for f in listdir(dir_path) if path.isfile(path.join(dir_path, f)) and f not in ["index.js", "_base.js"]]
        arr = []
        for f in files:
            split = f.split('_')[1].split('.')[0]
            split = split[0].capitalize() + split[1:]
            arr.append(split)

        return files, arr

    def updateModelName(self, fileName: str, modelName: str):
        if not fileName.startswith("_"):
            raise Exception(f"O {fileName} precisa começar começar com underline!")
        filePath = path.join(self.project_path, "src", "models", f"{fileName}.js")
        
        with open(filePath, 'r') as arquivo:
            linhas = arquivo.readlines()

        linha_desejada = linhas[5] 
        partes = linha_desejada.split("'")
        partes[1] = modelName
        nova_linha = "'".join(partes)

        linhas[5] = nova_linha

        with open(filePath, 'w') as arquivo:
            arquivo.writelines(linhas)

    def updateRouteName(self, routeName, oldRoute):
        filePath = path.join(self.project_path, "src", "index.js")
        
        with open(filePath, 'r', encoding='UTF-8') as arquivo:
            linhas = arquivo.readlines()

        line = self.getSpecificLine(oldRoute)
    
        if line == -1:
            return Exception(f"A rota {oldRoute} não foi encontrada no arquivo!")
        
        if line < 0 or line >= len(linhas):
            raise IndexError(f"A linha {line} está fora do intervalo válido (0 a {len(linhas) - 1}).")

        linha_desejada = linhas[line] 
        partes = linha_desejada.split("'")
        
        partes[1] = routeName
        nova_linha = "'".join(partes)

        linhas[line] = nova_linha

        with open(filePath, 'w', encoding='UTF-8') as arquivo:
            arquivo.writelines(linhas)

    def getSpecificLine(self, string) -> int:
        with open(path.join(self.project_path, "src", "index.js"), 'r', encoding='UTF-8') as file:
            lines = file.readlines()
            for i, line in enumerate(lines):
                if string in line:
                    return i
        return -1
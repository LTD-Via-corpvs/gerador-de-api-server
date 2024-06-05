from flask import Flask, request
from flask_cors import CORS
from manager import PackageManager, TemplateManager, ProjectManager
from os import getcwd

script_folder = getcwd()
package_manager = PackageManager(script_folder)
template_manager = TemplateManager(script_folder)
project_manager = ProjectManager(script_folder)

app = Flask(__name__)
CORS(app)

@app.get("/packages")
def get_packages():
    if len(package_manager.available_package_managers) == 0:
        return None, 204
    reload = False
    try:
        if 'reload' in request.args:
            reload = True
    except:
        return { 'error': 'Reload precisa ser um booleano.' }, 400
    if reload == True:
        return package_manager.reload(), 201
    return package_manager.available_package_managers, 200

@app.post("/api/generate")
def gen_handle():
    data = request.get_json()
    params = request.args
    if not data:
        return {"error": "No JSON data provided"}, 400
    if not params:
        return {"error": "No PARAMS data provided"}, 400
    
    if not 'project' in params:
        return { "error": "Specify the name of project" }, 400
    
    if not 'model' in data:
        return { "error": "Specify a name for model" }, 400
    if not 'file' in data:
        return { "error": "Specify a name for file" }, 400
    if not 'route' in data:
        return { "error": "Specify a name for route" }, 400
    
    project_name = params.get('project')
    modelName = data.get('model')
    fileName = data.get('file')
    junctionTable = data.get('junctionTable')
    routeName = data.get('route')

    try:
        project_manager.update_project_path(project_name)
        project_manager.insertImportIntoIndex(modelName)
        project_manager.generateModel(modelName, fileName, junctionTable)
        project_manager.generateController(modelName, fileName)
        project_manager.generateRoute(modelName, fileName)
        project_manager.insertRouteIntoIndex(modelName, routeName)
    except Exception as err:
        return { 'error': err.args[0] }, 500

    return {
        "message": "Model generated sucessfully!",
    }, 201

@app.post("/api/init")
def init_project():
    if not 'package_manager' in request.json:
        return 'É necessário específica o gerenciador de pacotes.', 400
    if not 'project_name' in request.json:
        return 'É necessário específica o nome do projeto.', 400
    _package_manager = request.json["package_manager"]
    project_name = request.json["project_name"]

    try:
        package_manager.init(project_name, _package_manager)
        package_manager.install(["express", "cors", "uuid", "cookie-parser", "bcrypt", "date-fns", "jsonwebtoken", "@prisma/client"], _package_manager)
        package_manager.install(["prisma", "nodemon"], _package_manager, True)
        package_manager.exec(["prisma", "init"])
        template_manager.update_project_path(project_name)
        template_manager.create_src()
        template_manager.create_index()
    except Exception as err:
        return { 'error': err.args[0] }, 500

    return { 'package_manager': _package_manager, 'project_name': project_name }, 200
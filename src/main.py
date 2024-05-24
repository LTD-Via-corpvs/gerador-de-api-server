from flask import Flask, request
from flask_cors import CORS
from manager import PackageManager, TemplateManager
from os import getcwd

script_folder = getcwd()
package_manager = PackageManager(script_folder)
template_manager = TemplateManager(script_folder)


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

@app.post("/init")
def init_project():
    if not 'package_manager' in request.json:
        return 'É necessário específica o gerenciador de pacotes.', 400
    if not 'project_name' in request.json:
        return 'É necessário específica o nome do projeto.', 400
    _package_manager = request.json["package_manager"]
    project_name = request.json["project_name"]

    try:
        package_manager.init(project_name, _package_manager)
        package_manager.install("express", _package_manager)

        template_manager.update_project_path(project_name)
        template_manager.create_env()
        template_manager.create_index()
    except Exception as err:
        return { 'error': err.args[0] }, 500

    return { 'package_manager': _package_manager, 'project_name': project_name }, 200
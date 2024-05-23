from flask import Flask, request
from flask_cors import CORS
from manager import PackageManager

package_manager = PackageManager()
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
import subprocess
import json
from os import chdir, getcwd, path, makedirs

class PackageManager:
    def __init__(self, script_folder) -> None:
        self.package_managers = ("yarn", "npm", "bun", "pnpm")
        self.available_package_managers = []
        self.script_folder = script_folder
        self.build_path = path.join(script_folder, '..', '..', 'build')
        print(script_folder)
        self.__check()

    def reload(self) -> list[str]:
        self.__check()
        return self.available_package_managers
    
    def init(self, project_name: str, package_manager: str = "npm") -> bool:
        result = False
        self.project_path = path.join(self.build_path, project_name)
        if path.exists(self.project_path):
            raise Exception("There is already a project with that name.")
        self.__check_info(package_manager)
        chdir(self.project_path)
        command = [package_manager]
        match package_manager:
            case "pnpm":
                command.extend(["init"])
            case _:
                command.extend(["init", "--y"])
        subprocess.run([*command], capture_output=True, text=True)
        self.__update_pkg()
        result = True
        chdir(self.script_folder)
        return result
    
    def install(self, dep: list[str], package_manager: str = "npm", dev: bool = False):
        self.__check_info(package_manager)
        chdir(self.project_path)
        command = [package_manager]
        match package_manager:
            case "pnpm":
                command.extend(["add", *dep])
            case "yarn":
                command.extend(["add", *dep])
            case "bun":
                command.extend(["add", *dep])
            case _:
                command.extend(["install", *dep])
        if dev:
            if package_manager == "npm" or package_manager == "pnpm":
                command.append("--save-dev")
            else:
                command.append("--dev")
        subprocess.run(command, capture_output=True, text=True)
        chdir(self.script_folder)

    def exec(self, command_list: list[str], package_manager: str = "npm"):
        self.__check_info(package_manager)
        chdir(self.project_path)
        command = [package_manager]
        match package_manager:
            case "pnpm":
                command.extend(["exec", *command_list])
            case "yarn":
                command.extend(["exec", *command_list])
            case "bun":
                command.extend(["exec", *command_list])
            case _:
                command.extend(["exec", *command_list])
        subprocess.run(command, capture_output=True, text=True)
        chdir(self.script_folder)

    def __check(self) -> None:
        if len(self.available_package_managers) > 0:
            self.available_package_managers.clear()
        for package_manager in self.package_managers:
            try:
                print(f"Checking: {package_manager}")
                result = subprocess.run([package_manager, '-v'], capture_output=True, text=True, check=True)
                print(f"{package_manager} version: {result.stdout.strip()}")
                self.available_package_managers.append(package_manager)
            except subprocess.CalledProcessError as e:
                print(f"Error running {package_manager}: {e}")
            except FileNotFoundError:
                print(f"{package_manager} not found in PATH")
            except Exception as e:
                print(f"Unexpected error checking {package_manager}: {e}")

    def __check_info(self, package_manager: str = "npm") -> None:
        if self.project_path is None:
            return Exception("PROJECT_PATH not defined.")
        if not path.exists(self.project_path):
            makedirs(self.project_path)
        if package_manager not in self.available_package_managers:
            return Exception("Package manager not available.")
    
    def __update_pkg(self):
        pkg_path = path.join(self.project_path, "package.json")
        with open(pkg_path, 'r+') as f:
            json_data = json.load(f)
            json_data["type"] = "module"
            json_data["scripts"] = {
                'dev': "nodemon src/index.js"
            }
            f.seek(0)
            json.dump(json_data, f, indent=4)
            f.truncate()

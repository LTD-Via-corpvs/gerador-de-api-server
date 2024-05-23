import subprocess
from os import chdir, getcwd

class PackageManager:
    def __init__(self) -> None:
        self.package_managers = ("yarn", "npm", "bun", "pnpm")
        self.available_package_managers = []
        self.project_path = None
        self.__check()

    def reload(self) -> list[str]:
        self.__check()
        return self.available_package_managers
    
    def set_project_path(self, project_path) -> None:
        self.project_path = project_path
    
    def init(self, package_manager: str = "npm") -> bool: # DONT TESTED
        result = False
        try:
            self.__check_info(package_manager)
            chdir(self.project_path)
            subprocess.run([package_manager, 'init', '--y'], capture_output=True, text=True)
            result = True
        except:
            pass
        finally:
            chdir(getcwd())
        return result
    
    def install(self, package_manager: str = "npm") -> bool: # DONT TESTED
        result = False
        try:
            self.__check_info(package_manager)
            chdir(self.project_path)
            command = [package_manager]
            match package_manager:
                case "pnpm", "yarn", "bun":
                    command.extend(["add", "express"])
                case _:
                    command.extend(["install", "express"])
            subprocess.run(command, capture_output=True, text=True)
            result = True
        except:
            pass
        finally:
            chdir(getcwd())
        return result

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

    def __check_info(self, package_manager: str = "npm") -> None: # DONT TESTED
        if self.project_path is None:
            return Exception("PROJECT_PATH not defined.")
        if package_manager not in self.available_package_managers:
            return Exception("Package manager not available.")
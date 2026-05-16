import sys
import subprocess
import os
from pathlib import Path

NAME = "Gdvc"
REPO = "maslina524/gdvc"
INSTALL_PATH = (Path("C:/Program Files") if os.name == "nt" else Path("/Applications")) / "gdvc"

ESC = "\x1b[0m"
RED_BOLD = "\x1b[1;31m"
BOLD = "\x1b[1m"
CYAN = "\x1b[36m"
GREEN = "\x1b[32m"

def panic(string: str):
    print(f"{RED_BOLD}{string}{ESC}")
    sys.exit(1)

def rprint(text: str, color: str): # rainbow print
    print(f"{color}{text}{ESC}")

def check_admin():
    if os.name == "nt":
        import ctypes
        return ctypes.windll.shell32.IsUserAnAdmin()
    else:
        return os.geteuid() == 0

def install_libs():
    failures = []
    try: import requests
    except: failures.append("requests")
    try: import tqdm
    except: failures.append("tqdm")

    if len(failures) == 0:
        return

    print("You do not have all the libraries installed")
    print(f"Install {", ".join(failures)} to continue [Y/n]? ", end="")

    answer = input().lower()
    if answer in ("n", "no"):
        panic("Installation canceled")
    elif answer not in ("y", "yes", ""):
        print("Invalid input, assuming Yes...")
    
    print()
    for lib in failures:
        print(f"----- {lib.upper()} -----")

        process = subprocess.Popen(
            [sys.executable, "-m", "pip", "install", lib],
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1
        )
        
        for line in process.stdout:
            print("  ", line, end='')
            sys.stdout.flush()
        process.wait()

        if process.returncode != 0:
            print(f"  Warning: {lib} installation failed")

        print()

def get_download_url():
    result = requests.get(f"https://api.github.com/repos/{REPO}/releases/latest")
    code = result.status_code
    if code < 200 and code >= 300:
        panic(f"From `api.github.com` the error code was returned: {code}")

    try:
        json_data = result.json()
        download_url = json_data.get("assets")[0].get("browser_download_url")
        if download_url != "":
            return download_url
        else:
            panic("Failed to find the download link")

    except Exception as e:
        panic(f"Parsing the json response from `api.github.com` failed\ntext: {result.text[:100]}...")

def download_file(url: str, path: Path):
    response = requests.get(url, stream=True)
    total_size = int(response.headers.get('content-length', 0))
    file_name = path.name
    with open(path, 'wb') as f:
        with tqdm(total=total_size, unit='B', unit_scale=True, desc=file_name) as progress_bar:
            for data in response.iter_content(chunk_size=1024):
                f.write(data)
                progress_bar.update(len(data))

def add_to_path(folder: Path):
    folder_str = str(folder)

    if os.name == "nt":
        import winreg
        import ctypes
        try:
            key = winreg.OpenKey(winreg.HKEY_LOCAL_MACHINE,
                                 r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
                                 0, winreg.KEY_READ | winreg.KEY_WRITE)
        except PermissionError:
            panic("Insufficient privileges to modify system PATH")

        try:
            current_path, reg_type = winreg.QueryValueEx(key, "PATH")
            if not isinstance(current_path, str):
                current_path = ""
        except FileNotFoundError:
            current_path = ""
            reg_type = winreg.REG_EXPAND_SZ

        paths = [p.strip() for p in current_path.split(";") if p.strip()]
        if folder_str in paths:
            rprint(f"{folder_str} already in global PATH", GREEN)
            winreg.CloseKey(key)
            return

        new_path = current_path + (";" if current_path else "") + folder_str
        winreg.SetValueEx(key, "PATH", 0, reg_type, new_path)
        winreg.CloseKey(key)

        HWND_BROADCAST = 0xFFFF
        WM_SETTINGCHANGE = 0x001A
        ctypes.windll.user32.SendMessageW(HWND_BROADCAST, WM_SETTINGCHANGE, 0, "Environment")
        rprint(f"Added {folder_str} to global PATH", GREEN)

    else:
        profile_dir = Path("/etc/profile.d")
        if profile_dir.exists():
            sh_file = profile_dir / "gdvc_path.sh"
            content = f'export PATH="{folder_str}:$PATH"\n'
            if sh_file.exists() and folder_str in sh_file.read_text():
                rprint(f"{folder_str} already in global PATH", GREEN)
                return
            
            sh_file.write_text(content)
            sh_file.chmod(0o755)
            rprint(f"Added {folder_str} to global PATH via {sh_file}", GREEN)
        else:
            env_file = Path("/etc/environment")
            if env_file.exists():
                lines = env_file.read_text().splitlines()
                new_lines = []
                found = False
                for line in lines:
                    if line.startswith("PATH="):
                        existing = line[5:].strip('"')
                        if folder_str in existing:
                            rprint(f"{folder_str} already in global PATH", GREEN)
                            return
                        new_path = f"{folder_str}:{existing}" if existing else folder_str
                        new_lines.append(f'PATH="{new_path}"')
                        found = True
                    else:
                        new_lines.append(line)
                if not found:
                    new_lines.append(f'PATH="{folder_str}"')

                env_file.write_text("\n".join(new_lines))
                rprint(f"Added {folder_str} to global PATH via /etc/environment", GREEN)
            else:
                panic("Cannot find /etc/profile.d or /etc/environment for global PATH modification")
        
if __name__ == "__main__":
    install_libs()
    import requests
    from tqdm import tqdm

    if not check_admin():
        panic("Administrator privileges (sudo) are required")

    rprint(f"Download to `{INSTALL_PATH}`", CYAN)

    download_url = get_download_url()
    if not INSTALL_PATH.exists():
        os.mkdir(INSTALL_PATH)
    download_file(download_url, INSTALL_PATH / "gdvc.exe")

    rprint(f"The {NAME} has been successfully installed!", GREEN)
    print()

    rprint(f"Installing {NAME} to the PATH", CYAN)
    add_to_path(INSTALL_PATH)
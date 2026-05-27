import sys
from pathlib import Path
import re
import subprocess

ESC = "\x1b[0m"
BOLD = "\x1b[1m"
ITALIC = "\x1b[3m"

def re_line(line: str) -> str:
    ret = re.sub(r'`(.*?)`', rf'{ITALIC}\1{ESC}', line, flags=re.DOTALL)
    return ret

def re_line_adoc(line: str) -> str:
    ret = re.sub(r'`(.*?)`', rf'__\1__', line, flags=re.DOTALL)
    return ret

def save_to_txt(data: str, path: Path) -> None:
    lines = data.split("\n")
    ret = []
    is_code = False
    for line in lines:
        if line.startswith("# "):
            ret.append(f"{BOLD}{line[2:]}{ESC}\n")
            ret.append(f"{'=' * len(line[2:])}\n")

        elif line.startswith("## "):
            ret.append(f"\n{BOLD}{line[3:]}{ESC}\n")

        elif line.startswith("```"):
            is_code = not is_code
            if is_code:
                ret.append(f"{ITALIC}")
            else:
                ret.append(f"{ESC}")
            
        else:
            if line != "":
                ret.append(f"{re_line(line)}\n")
    
    output = "".join(ret)
    with open(f"doc/txt/{path.name[:-3]}.txt", "w") as f:
        f.write(output)

def save_to_html(data: str, path: Path) -> None: # use asciidoc
    lines = data.split("\n")
    ret = []
    is_code = False
    for line in lines:
        if line.startswith("# "):
            ret.append(f"= {line[2:]}\n")

        elif line.startswith("## "):
            ret.append(f"\n== {line[3:]}\n")

        elif line.startswith("```"):
            is_code = not is_code
            
        else:
            if line != "":
                if is_code:
                    ret.append(f"`{re_line_adoc(line)}`\n")
                else:
                    ret.append(f"{re_line_adoc(line)}\n")
    
    output = "".join(ret)
    with open(f"doc/adoc/{path.name[:-3]}.adoc", "w") as f:
        f.write(output)

    subprocess.run(f"asciidoctor -o doc/html/{path.name[:-3]}.html doc/adoc/{path.name[:-3]}.adoc", shell=True)

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Input file path expected!")
        sys.exit(1)

    input_file = Path(sys.argv[1])

    with open(input_file, "r") as f:
        data = f.read()

    save_to_txt(data, input_file)
    save_to_html(data, input_file)
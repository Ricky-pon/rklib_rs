import json
import argparse
import tempfile
import subprocess

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Expander")
    parser.add_argument("modules", nargs="*", help="Module List")
    opts = parser.parse_args()

    with open("contents.json", "r") as f:
        contents = json.load(f)

    st = list(map(lambda x: contents[x]["module"], opts.modules.copy()))
    module_list = set()
    while len(st) != 0:
        module = st.pop()
        module_list.add(module)
        if "dependency" in contents[module]:
            for dep_module in contents[module]["dependency"]:
                st.append(dep_module)

    output_data = []
    for module in module_list:
        module_path = contents[module]["path"]

        with open(module_path, "r") as f:
            output_data.append("pub mod {} {{".format(module))

            for line in f:
                output_data.append(line.rstrip())

            output_data.append("}\n")

    for module in module_list:
        output_data.append('use {}::*;'.format(module))

    # rustfmt and output
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_file = temp_dir + '/output.rs'
        with open(temp_file, 'w') as f:
            for line in output_data:
                print(line, file=f)
        output_data = subprocess.run(["rustfmt", temp_file], check=True)

        with open(temp_file, 'r') as f:
            lines = f.readlines()
            i = 0
            while i < len(lines):
                if '#[cfg(test)]' in lines[i]:
                    i += 2
                    nest_depth = 1
                    while nest_depth != 0:
                        if '{' in lines[i]:
                            nest_depth += 1
                        if '}' in lines[i]:
                            nest_depth -= 1
                        i += 1
                elif '//' in lines[i]:
                    i += 1
                else:
                    print(lines[i], end="")
                    i += 1
import os
import sys
import datetime

def main(input_root, output_root):
    for root, dirs, files in os.walk(input_root):
        for file in files:
            if file.endswith('.crate'):
                path = os.path.join(root, file)
                oroot = root[len(input_root):]
                output_path = os.path.join(output_root, oroot, file)
                if os.path.exists(output_path):
                    print(f"Already exists: {output_path}")
                else:
                    command = f"tar -xzf {path} -C {output_path} --wildcards --no-anchored '*.rs' --wildcards Cargo.toml"
                    # Create output directory.
                    os.makedirs(output_path, exist_ok=True)
                    # Extract all *.rs files from the crate.
                    print(datetime.datetime.now(), command)
                    exit_code = os.system(command)
                    if exit_code != 0:
                        print(f"Failed with error: {exit_code}")

if __name__ == '__main__':
    if len(sys.argv) < 3:
        print('Usage: ungzip.py <input_root> <output_root>')
        sys.exit(1)
    main(sys.argv[1], sys.argv[2])

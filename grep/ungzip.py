import os
import sys

def main(input_root, output_root):
    for root, dirs, files in os.walk(input_root):
        for file in files:
            if file.endswith('.crate'):
                path = os.path.join(root, file)
                output_path = os.path.join(output_root, file)
                command = f"tar -xzf {path} --wildcards '*.rs' -C {output_path}"
                # Create output directory.
                os.makedirs(os.path.dirname(output_path), exist_ok=True)
                # Extract all *.rs files from the crate.
                os.system(command)


if __name__ == '__main__':
    if len(sys.argv) < 3:
        print('Usage: ungzip.py <input_root> <output_root>')
        sys.exit(1)
    main(sys.argv[1], sys.argv[2])
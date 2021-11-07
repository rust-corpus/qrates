#!/usr/bin/python3

import json
import os

from shutil import rmtree
from subprocess import run, PIPE

TEST_DIR = os.path.abspath(os.path.dirname(__file__))
OUTPUT_DIR = os.path.join(TEST_DIR, 'rust-corpus')
RUSTC = os.path.join(TEST_DIR, '../target/debug/rustc')
STATUS_PRINTER = os.path.join(TEST_DIR, '../target/debug/print-stats')

def run_extractor():
    rmtree(OUTPUT_DIR, ignore_errors=True)
    os.mkdir(OUTPUT_DIR)
    env = os.environ.copy()
    env['SYSROOT'] = run(['rustc', '--print', 'sysroot'], stdout=PIPE).stdout.strip()
    env['RUSTC'] = RUSTC
    env['CORPUS_RESULTS_DIR'] = OUTPUT_DIR
    env['CORPUS_OUTPUT_JSON'] = 'true'
    assert run(['cargo', 'clean'], cwd=TEST_DIR).returncode == 0
    assert run(['cargo', 'check'], env=env, cwd=TEST_DIR).returncode == 0

class Tables:

    def __init__(self, file_name):
        self.data = json.load(open(file_name))

    def counter(self, name):
        return self.data['counters'][name]

    def relation(self, name):
        return self.data['relations'][name]['facts']

    def interning_table(self, name):
        return self.data['interning_tables'][name]['contents']

    def __getattr__(self, attr):
        if attr in self.data['relations']:
            return self.relation(attr)
        elif attr in self.data['counters']:
            return self.counter(attr)
        elif attr in self.data['interning_tables']:
            return self.interning_table(attr)
        else:
            raise KeyError(attr)

def check_output():
    files = list(sorted(os.listdir(OUTPUT_DIR)))
    assert len(files) == 2
    output_file = os.path.join(OUTPUT_DIR, files[1])
    tables = Tables(output_file)

    assert len(tables.builds) == 1

    assert tables.modules == 3

    strings = '''
    # Static.
    extractor_test.X

    extractor_test.test1
    extractor_test.test2

    extractor_test.S
    # impl S
    extractor_test.implement.test7
    extractor_test.implement.test8

    # trait T
    extractor_test.T
    extractor_test.T.test3
    extractor_test.T.test4
    extractor_test.T.test5
    extractor_test.T.test6

    # impl T for S
    extractor_test.implement_1
    extractor_test.implement_1.test3
    extractor_test.implement_1.test4
    extractor_test.implement_1.test5
    extractor_test.implement_1.test6

    extractor_test.module1
    extractor_test.module1.module2

    extractor_test.module1.module2.test2

    extractor_test.module1.module2.S
    extractor_test.module1.module2.implement.test7
    extractor_test.module1.module2.implement.test8

    # trait T
    extractor_test.module1.module2.T
    extractor_test.module1.module2.T.test3
    extractor_test.module1.module2.T.test4
    extractor_test.module1.module2.T.test5
    extractor_test.module1.module2.T.test6

    # impl t for S
    extractor_test.module1.module2.implement_module2.test3
    extractor_test.module1.module2.implement_module2.test4
    extractor_test.module1.module2.implement_module2.test5
    extractor_test.module1.module2.implement_module2.test6
    '''.strip().splitlines()

    for string in strings:
        string = string.strip()
        if string.startswith('# '):
            continue
        assert string in tables.strings, string
    assert len(tables.relation('terminators_call')) == 18

    # for t in sorted(tables.strings):
        # if t.startswith('extractor_test.'):
            # print(t)
    # print(json.dumps(tables.data['counters'], indent=2, sort_keys=True))
    # print(json.dumps(tables.data['interning_tables'], indent=2, sort_keys=True))

def main():
    run_extractor()
    check_output()


if __name__ == '__main__':
    main()

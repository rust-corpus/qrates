#!/usr/bin/python3

import datetime
import json
import os
import subprocess


ROOT = os.path.abspath('.')
DATA_PATH = os.path.join(ROOT, 'data')
CRATES_INFO_PATH = os.path.join(DATA_PATH, 'crate-information.json')
LOG_PATH = os.path.join(DATA_PATH, 'compile.log')
COMPILATION_PATH = os.path.join(DATA_PATH, 'compilation')
MANIFEST_PATH = os.path.join(COMPILATION_PATH, 'Cargo.toml')
SCCACHE_DIR = os.path.join(DATA_PATH, 'cache')
EXTRACTOR_PATH = os.path.join(ROOT, 'rustql-extractor/target/release/rustc')
EXTRACTOR_TARGET_DIR = os.path.join(DATA_PATH, 'crates')


def collect_crates():
    with open(CRATES_INFO_PATH) as fp:
        return json.load(fp)


def create_manifest(crate):
    with open(MANIFEST_PATH, 'w') as fp:
        fp.write('''
[package]
name = "rustql-dummy"
version = "0.1.0"
edition = "2018"

[dependencies]
{} = "{}"
'''.format(crate['name'], crate['version']))


def compile_crate(crate):
    result = subprocess.run(
        args=['/home/vagrant/.cargo/bin/cargo', 'build', '--verbose'],
        # Give 20 minutes for each crate.
        timeout=20*60,
        cwd=COMPILATION_PATH,
        env={
            # "RUST_BACKTRACE": "1",
            "PATH": "/usr/local/sbin:/usr/local/bin:"
                    "/usr/sbin:/usr/bin:/sbin:/bin",
            "SCCACHE_DIR": SCCACHE_DIR,
            "EXTRACTOR_TARGET_DIR": EXTRACTOR_TARGET_DIR,
            # "LD_LIBRARY_PATH": "/data/toolchain/lib/",
            "RUSTC_WRAPPER": "sccache",
            "RUSTC": EXTRACTOR_PATH,
        },
    )


def main():
    crates = collect_crates()
    with open(LOG_PATH, 'a') as fp:
        def log(*args):
            timestamp = datetime.datetime.utcnow().strftime("%Y-%m-%d %H:%M:%S")
            fp.write(timestamp)
            fp.write(',')
            fp.write(','.join(args))
            fp.write('\n')
            fp.flush()
        for crate in crates:
            log('START_COMPILE', crate['id'])
            try:
                create_manifest(crate)
                compile_crate(crate)
            except Exception as e:
                log('ERROR', str(e))
                raise
            log('END_COMPILE', crate['id'])
            break


if __name__ == '__main__':
    main()

# Extracting Data from Crates Published on crates.io

This section shows how to extract data from crates published on [crates.io](https://crates.io/).

**Warning:** The extraction involves compiling the crates and, therefore, may result in running potentially malicious code on your system. Therefore, make sure to compile the code on a virtual machine or in some other throw-away environment.

This section assumes that you have already compiled Qrates. You can find the instructions how to do that [here](building.md).

## Obtaining the List of Packages

The second step is to select the packages[^package] from which we want to extract the data. This list of packages should be stored in the file `CrateList.json`. For example, if you want to analyse the package `rand`, create `CrateList.json` with the following contents:

[^package]: A package uploaded on crates.io can contain several crates. For example, it is common for executables to be split into an executable crate and a library crate.

```json
{
  "creation_date": {
    "secs_since_epoch": 1579271281,
    "nanos_since_epoch": 847419904
  },
  "crates": [
    {
      "Package": {
        "name": "rand",
        "version": "0.7.3"
      }
    }
  ]
}
```

If you want to analyse all packages on crates.io, you can rename `CrateList-all-2020-01-14.json` to `CrateList.json`. That file contains the latest versions of all packages that were published on crates.io on 2020-01-14. Please note that compiling all packages requires at least 1 TB of hard drive space and running queries on so large dataset may require up to 200 GB of RAM. The dataset `CrateList-top-200-2020-01-17.json` 200 packages that were the most downloaded on crates.io; this dataset should be analysable on a laptop with 8 GB of RAM.

You can also create the list with the latest versions of all packages by running the following command:

```bash
cargo run --release -- init-all
```

## Compiling the Packages

*Note:* Instead of compiling yourself, you can also download the extracted data from [here](https://doi.org/10.5281/zenodo.4026639).

Qrates uses the [Rustwide](https://github.com/rust-lang/rustwide/) library for compiling packages. Please see the Rustwide documentation for the system requirements; most importantly you need to have [Docker](https://www.docker.com/) installed (you can find the installation instructions [here](https://docs.docker.com/engine/install/)).

You can start the compilation as follows:

```bash
mkdir ../workspace
cargo run --release -- compile
```

This command may fail with a permission error if the user does not have the necessary permissions to communicate with the Docker manager. In that case, use `sudo`:

```bash
sudo $(which cargo) run --release -- compile
```

Attempting to compile all packages from crates.io on Lenovo ThinkPad T470p takes about a week. You can check how many packages already successfully compiled by running the following command:

```bash
ls ../workspace/rust-corpus/*/success | wc -l
```

*Note:* it is likely that the number of successfully compiled packages will be smaller than the one we reported in the paper because some of the packages were removed from crates.io in the meantime.

## Checking Compilation Results

The overview of compilation errors can be seen by running the following command:

```bash
cargo run --release -- check-compilation
```

It will print the statistics of how many crates failed to compile for some common reason, how many failed likely due to a bug in the extractor, and how many failed for yet unrecognised reason. It will also print 5 paths of each of the latter groups.

*Note:* The classification is implemented in `manager/src/compilation_checks.rs`.

If you are using an older `CrateList`, it is very likely that many crates will fail to compile because their dependencies were removed from the registry (they were “yanked”). One workaround for this problem would be to fork [the crates.io registry](https://github.com/rust-lang/crates.io-index) and then restore the removed crates by setting their `yanked` flag to `False`. It is also recommended to remove all package versions that are newer than the ones in the the crate list. Both these operations could be done by executing the following script from the root directory of the registry repository:

```python
#!/usr/bin/python3

import json
import os

CRATE_LIST = '<path-to>/CrateList-all-2020-01-14.json'

def rewrite(path, package, versions):
    """Rewrite the cargo registry entry for `package` to contain only
    the entries older than the one mentioned in `versions` and restore
    all yanked version.
    """
    if package not in versions:
        # This package probably was published after we created the
        # crates list and, therefore, will not appear among
        # dependencies.
        return
    newest_version = versions[package]
    with open(os.path.join(path, package)) as fp:
        try:
            lines = fp.read().splitlines()
        except:
            print(path, package)
            raise
    with open(os.path.join(path, package), 'w') as fp:
        for line in lines:
            data = json.loads(line)
            if 'yanked' in data and data['yanked']:
                data['yanked'] = False
                json.dump(data, fp, separators=(',', ':'))
            else:
                fp.write(line)
            fp.write('\n')
            if data['vers'] == newest_version:
                break

def main():
    with open(CRATE_LIST) as fp:
        crates = json.load(fp)['crates']
        versions = dict(
            (crate['Package']['name'], crate['Package']['version'])
            for crate in crates
        )

    for root, dirs, files in os.walk('.'):
        if root != '.':
            for package in files:
                rewrite(root, package, versions)
        else:
            dirs.remove('.git')

if __name__ == '__main__':
    main()
```

The registry that matches `CrateList-all-2020-01-14.json` can be found [here](https://github.com/vakaras/crates.io-index). To try recompiling all failed packages with this registry, execute the following commands:

```bash
cargo run --release -- check-compilation --delete-failures
cargo run --release -- compile --purge-build-dir --custom-cargo-registry https://github.com/vakaras/crates.io-index
```

## Moving Extracted Data

Since the extraction phase has quite different technical requirements from the later phases, it is common to execute these phases on different machines. The following command can be used to move deduplicated extracted data to a new directory for easy transfer:

```bash
cargo run --release -- move-extracted <target-dir>
```

The command sleeps for 20 seconds after it collects the list of files to move and performing the actual move to reduce the risk of moving half-written files.

It will also generate `files.json` files that are then used by one of the queries to select the builds for analysis. Please note that some packages (for example, `sheesy-cli-4.0.7` and `actix-net-0.3.0`) are empty when compiled with default features, which results in `files.json` being empty.

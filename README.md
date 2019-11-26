# Branch

### What?
Branch is a simple utility that recursively retrieves paths from 
the provided directory. It's goal is to be as fast or faster than
`ls -R` or `fine *`. This goal has not yet been attained.

### How?
Simply run `branch`.

```
$ branch
/home/kiedtl/src/branch/test/
/home/kiedtl/src/branch/test/anothertree
/home/kiedtl/src/branch/test/tree
/home/kiedtl/src/branch/test/etc
```

```
$ branch --help
branch 0.1.0
Kied Llaentenn
recursively get paths quickly.

USAGE:
	branch [FLAGS] [OPTIONS] [PATH]

FLAGS:
	-a, --all	Print hidden items.
	-c, --count	Print count of dirs and files.
	-h, --help	Print help information.
	-s, --sort	Sort files. Decreases performance.
	-d, --dirs	Print only directories.
	-V, --version	Print version information.

OPTIONS:
	-l, --level <level>	Maximum directory level to recurse into.

ARGS:
	<PATH>	Input directory to use. Default is current directory.
```

### Where?
#### Build Dependencies
To build from source, you will need:
	- the Rust compiler toolchain
	- [just](https://github.com/casey/just)

#### From source
To install, first clone the repository:
```
$ git clone https://github.com/lptstr/branch
```
Then, build release and install.
```
$ just clean
$ just release
$ just install
```

*in the future, binaries will be provided.*

### Why?
`\_(°-°)_/`
Life can be boring 

### License
- This program is licensed under the ISC License.
- See `LICENSE.md` for more information.

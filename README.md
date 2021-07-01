# zc

Faster `cd` To the Directory. Instead of `cd` with path, `zc` can jump to destination, even if it is just the directory name only.

**The more you use it, the more he understands you**

*It works by maintaining a database, directories must be visited first before they can be jumped to.*



## INSTALLATION

Follow below commands to install `zc`

```shell
git clone https://github.com/Jamyw7g/zc.git
cd zc
cargo install --path .
python3 install.py
```



## USAGE

- show help

```shell
>>> zc -h
zc 1.0.0
Usage: zc [OPTIONS] [PATH...]

Options:
    -a, --add PATH      add new path to database with default weight
    -i, --increase      increase current directory
    -d, --decrease      decrease current directory
    -s, --stat          show database, which contain path and weight
        --purge         remove any non-existent paths from database
    -h, --help          print this help
    -v, --version       show version information
```



- Jump To A Directory That Contains `foo`:

```shell
z foo
```

- Or add more parameters to improve accuracy

```shell
z foo bar
```

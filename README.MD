# Hosts Updater

A simple command line tool for updating your hosts file. This software can compress large hosts files.

```commandline
A piece of software for updating hosts files

Usage: update_hosts2.exe [OPTIONS] --urls <urls>...

Options:
  -u, --urls <urls>...                   A list of files to download
  -o, --out <out>                        Where to write the file to [default: hosts]
  -c, --compression_level <compression>  The number of columns to use [default: 9]
  -i, --ignore [<ignore>...]             Select urls/files to use as an ignore list
  -h, --help                             Print help information
  -V, --version                          Print version information
```

## Examples

Let's say you want to update to add an ignore filter to your current hosts file:

```commandline
> update_hosts2 --urls file:///etc/hosts --out /etc/hosts --ignore file://hosts
```

Let's say you want to merge your current hosts file with [Steven Black's Hosts](https://github.com/StevenBlack/hosts)

```commandline
> update_hosts2 --urls https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts file:///etc/hosts --out /etc/hosts
```

Let's say you wanted to only block gambling websites using Steven Black's Hosts:

```commandline
> update_hosts2 --urls https://raw.githubusercontent.com/StevenBlack/hosts/master/alternates/gambling/hosts file:///etc/hosts --out /etc/hosts --ignore https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts
```

Let's say you wanted to use Steven Black's Hosts on Windows (Level 9 Compression required for performance):

```commandline
> update_hosts2 --urls https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts --out C:\Windows\System32\drivers\etc\hosts -c 9
```

# Hostfile Updater
A simple tool to add and remove entries from the host file (i.e., /etc/hosts) on Linux.

## Installation
```bash
cargo install https://github.com/hannes-hochreiner/hostfile-updater
```

## Help
```bash
hostfile-updater -h
```

## Examples
### Add one entry
```nu
open /etc/hosts | hostfile-updater add 192.168.0.1 test.localdomain | sudo save /etc/hosts
```
### Add all combinations from a configuration file
```toml
addresses = ["127.0.0.1", "::1"]
hostnames = ["test test.localdomain"]
```
```nu
open /etc/hosts | hostfile-updater add-config ./examples/config.toml | sudo save /etc/hosts
```

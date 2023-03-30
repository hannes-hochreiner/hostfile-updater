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

### Add multiple entries
```nu
open /etc/hosts | hostfile-updater add 192.168.0.1,::1 test1.localdomain,test2.localdomain | sudo save /etc/hosts
```

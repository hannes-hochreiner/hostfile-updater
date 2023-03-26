# Hostfile Updater
A simple tool to add and remove entries from the host file (i.e., /etc/hosts) on Linux.

# Installation
```bash
cargo install https://github.com/hannes-hochreiner/hostfile-updater
```

# Example
```nu
open /etc/hosts | hostfile-updater add 192.168.0.1 test.localdomain | sudo save /etc/hosts
```

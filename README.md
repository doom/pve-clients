# pve-clients

Proxmox VE API clients, generated from the [official JSONSchema spec](https://pve.proxmox.com/wiki/Proxmox_VE_API#JSON_and_JSON_Schema).

## Supported languages

- [ ] Go
- [x] Python
- [x] Rust

## Supported Proxmox versions

Current generated clients:
- `8.2.0`

Compatible with the current generators (see below for instructions on how to generate):
- `7.2-1`

## Generating clients locally

```bash
cd generate
```

Install requirements:
```bash
make venv && source .venv/bin/activate && pip3 install -r requirements.txt
```

Run generation:
```bash
make rust
```
> See `make help` for a list of available targets.

Run generation for a specific Proxmox version / Debian distribution:
```bash
make PROXMOX_VERSION=7.2-1 DEBIAN_DISTRO=bullseye rust
```

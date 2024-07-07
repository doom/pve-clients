.DEFAULT_GOAL = help

## Display this help message
help:
	@cat $(MAKEFILE_LIST) | awk '							\
	BEGIN { FS = ":"; printf "Usage: make \033[36m<target>\033[0m\n\n" }		\
	/^## / {									\
		sub(/##[ \t]*/, "", $$0) ;						\
		help = help (help ? "\n\t" : "") $$0 ;					\
		next ;									\
	}										\
	/^[^ \t]+:/ {									\
		if (help) printf ("  \033[36m" "%-25s" "\033[0m-- " help "\n", $$1 )	\
	}										\
	/.*/ { help = "" }'

DUMP_SCHEMA_DIR	:= dump-schema
PROXMOX_VERSION	?= 7.2-1
DUMP_SCHEMA	:= proxmox-dump-schema:$(PROXMOX_VERSION)
DEBIAN_DISTRO	?= bullseye
SCHEMA_FILE	?= PVE-schema-$(PROXMOX_VERSION).json

## Dump the JSONSchema for the API
schema:

$(SCHEMA_FILE) schema:
		docker build --platform=linux/amd64 \
			-t $(DUMP_SCHEMA) \
			--build-arg DISTRO=$(DEBIAN_DISTRO) \
			--build-arg PROXMOX_VERSION=$(PROXMOX_VERSION) \
			$(DUMP_SCHEMA_DIR)
		docker run $(DUMP_SCHEMA) bash -c "/app/dump-schema.pl" > $(SCHEMA_FILE)

VENV		:= .venv

$(VENV):
		virtualenv $(VENV)

## Initialize a virtual environment
venv:		$(VENV)

## Generate the implementation for Rust
rust:		$(SCHEMA_FILE)
		echo "Generating Rust implementation from $(SCHEMA_FILE)"
		./generate.py $(SCHEMA_FILE) rust --enable-async-client -o ../rust/src

.PHONY:		help schema venv rust

ARG DISTRO=bookworm

FROM debian:${DISTRO}

ARG DISTRO=bookworm

RUN apt-get update && apt-get install -y wget

RUN mkdir -p /etc/apt/sources.list.d && \
    echo "deb http://download.proxmox.com/debian/pve ${DISTRO} pve-no-subscription" >/etc/apt/sources.list.d/pve-install-repo.list && \
    wget https://enterprise.proxmox.com/debian/proxmox-release-${DISTRO}.gpg -O /etc/apt/trusted.gpg.d/proxmox-release-${DISTRO}.gpg && \
    apt-get update

ARG PROXMOX_VERSION

RUN apt-get install -y proxmox-ve=${PROXMOX_VERSION} || true

COPY . /app

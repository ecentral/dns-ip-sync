# Hetzner DNS



```yml

version: '3.5'

services:
  hetzner:
    image: ecentral/hetzner-dns-ip-sync
    network_mode: host
    environment:
      HETZNER_ZONE: "your-zone-name"
      HETZNER_DOMAIN: "your-domain.com"
      HETZNER_TOKEN: "your-hetzner-dns-token"

```
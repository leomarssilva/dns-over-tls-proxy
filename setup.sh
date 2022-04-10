#!/bin/sh

# alias docker=podman

cd dns-proxy/

podman build --tag test-dns-proxy .

cd ../test-infra/server/

podman build --tag coredns .

cd ../certificates/
podman secret create coredns_private_key server.key
podman secret create coredns_certificate server.crt
podman secret create coredns_certificate_ca ca.crt

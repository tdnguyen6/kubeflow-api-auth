#!/usr/bin/env zsh

sudo podman run --rm -p 5004:80 -v ./nginx.conf:/etc/nginx/nginx.conf:ro docker.io/library/nginx

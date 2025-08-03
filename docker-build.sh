#!/bin/bash

# Build docker image
docker build -t mortar .

# Tag the image
docker tag latest docker-registry.redtek.dev/mortar

# Push the image
docker push dockerregistry.redtek.dev/mortar

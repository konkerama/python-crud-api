#!/bin/bash

docker build -t konkerama/k8s-application:latest .
docker push konkerama/k8s-application:latest

#!/usr/bin/env bash
. env.sh

docker volume rm ${VOLUME_NAME}
docker plugin disable vieux/sshfs:latest
docker plugin rm vieux/sshfs:latest

echo "Remove he ${VOLUME_NAME} and sshfs plugin"
#!/usr/bin/env bash
echo "Enter the enviroment file name"
read env_file

# check if the env file exsits
if [ -f ${env_file} ]; then
    echo "activate with file ${env_file}"
    set -a
    . ${env_file}
    # export $(grep -v '^#' .env | xargs -d '\n')
    set +a
    # source ${env_file}
else
    echo "${env_file} not found"
    exit 1
fi

echo "install sshfs pluging with sshkey enabled from ${SSH_KEY}"
docker plugin install --grant-all-permissions vieux/sshfs sshkey.source=${SSH_KEY}

echo "create the ${VOLUME_NAME} from ${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH}"
docker volume create -d vieux/sshfs -o sshcmd=${REMOTE_USER}@${REMOTE_HOST}:${REMOTE_PATH} ${VOLUME_NAME}

echo "You have a volume avalable"
docker volume ls
docker volume inspect ${VOLUME_NAME}

sleep 2;

echo "now test with busy box volume"
docker run -it --rm --name test-remote-volume -v  ${VOLUME_NAME}:/tmp/test/ busybox ls /tmp/test

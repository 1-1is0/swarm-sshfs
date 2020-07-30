USER="docker"
REMOTE_USER="user10"
REMOTE_HOST="guud.space"


echo "install sshfs pluging with sshkey enabled"
docker plugin install vieux/sshfs sshkey.source=/home/${USER}/.ssh/

echo "create the remote-volume"
docker volume create -d vieux/sshfs -o sshcmd=${REMOTE_USER}@${REMOTE_HOST}:/home/${REMOTE_USER}/data  remote-volume


#!/bin/bash

#Install Docker
apt-get remove docker docker-engine docker.io containerd runc -y
apt update -y
apt install ca-certificates curl gnupg lsb-release
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

apt update
apt install docker-ce docker-ce-cli containerd.io -y

#Create a new user and give it permission to use Docker even if not root
useradd container
usermod -aG docker container

#We setup the docker daemon to allow sending images to an HTTP registry
touch /etc/docker/daemon.json
insecure="{
  \"insecure-registries\" : [\"faraday.inria.fr:5000\"]
}"
echo $insecure | tee -a /etc/docker/daemon.json > /dev/null

echo "docker exec -i container \"$@\"" | tee /bin/r2 > /dev/null
chmod +x /bin/r2

echo "# launch docker bash if logging in through SSH
if [[ -n \$SSH_CONNECTION ]] ; then
    #extracts the name of the first container on the list. There should be only one per machine anyway.
    container_name=$(docker ps | sed '2q;d' | cut -d" " -f32)
    #if there is no containers running currently
    if [ \"\$container_name\" == \"\" ]; then
        echo \"There is no container currently running on this machine.\"
        docker container ls -a
        exit
    fi
    docker exec -it \$container_name bash
    exit
fi" | tee -a /home/container/.profile > /dev/null
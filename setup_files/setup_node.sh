#!/bin/bash

#Install Docker
apt-get remove docker docker-engine docker.io containerd runc -y
apt update -y
apt install ca-certificates curl gnupg lsb-release
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

apt install docker-ce docker-ce-cli containerd.io

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


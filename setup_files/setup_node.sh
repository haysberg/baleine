#!/bin/bash

#Install Docker
apt-get remove docker docker-engine docker.io containerd runc -y
apt update -y
apt upgrade -y
apt install ca-certificates curl gnupg lsb-release -y
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

apt update -y
apt install docker-ce docker-ce-cli containerd.io -y

#Create a new user and give it permission to use Docker even if not root
useradd container
usermod -aG docker container
mkhomedir_helper container

#We setup the docker daemon to allow sending images to an HTTP registry
touch /etc/docker/daemon.json
insecure="{
  \"insecure-registries\" : [\"faraday.inria.fr:5000\", \"faraday:5000\"]
}"
echo $insecure | tee -a /etc/docker/daemon.json > /dev/null

wget https://raw.githubusercontent.com/haysberg/r2dock/main/setup_files/r2
mv ./r2 /bin/r2
chmod +x /bin/r2

echo "/bin/r2" >> /etc/shells

passwd -d container
chsh --shell /bin/r2 container
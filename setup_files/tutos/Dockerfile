FROM ubuntu:latest

# Install necessary binaries for tutorials
RUN apt update -y && apt upgrade -y && apt install iputils-ping openssh-client openssh-server bash netcat iproute2 net-tools -y

# Setup Bash prompt to not get lost when in interactive mode
RUN echo "PS1='[CONTAINER]${debian_chroot:+($debian_chroot)}\u@\h:\w\$ '" | tee /root/.bashrc > /dev/null
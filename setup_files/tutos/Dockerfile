FROM ubuntu:latest

# Install necessary binaries for tutorials
RUN apt update -y && apt upgrade -y && apt install iputils-ping openssh-client openssh-server bash netcat iproute2 net-tools tcpdump dnsutils netcat nano vim -y

# Setup Bash prompt to not get lost when in interactive mode
RUN echo "PS1='[CONTAINER] ${debian_chroot:+($debian_chroot)}\u@\h:\w\$ '" | tee /root/.bashrc > /dev/null

#Setting up OpenSSH server
RUN sed -ri 's/^#?PermitRootLogin\s+.*/PermitRootLogin yes/' /etc/ssh/sshd_config
RUN sed -ri 's/UsePAM yes/#UsePAM yes/g' /etc/ssh/sshd_config
RUN sed -i 's/#Port 22/Port 2222/g' /etc/ssh/sshd_config
RUN sed -i 's/#ListenAddress 0.0.0.0/ListenAddress 127.0.0.1/g' /etc/ssh/sshd_config
RUN echo "auth sufficient pam_permit.so" | tee -a /etc/pam.d/sshd >> /dev/null

RUN mkdir -p /run/sshd
RUN echo "PermitEmptyPasswords yes" | tee -a /etc/ssh/sshd_config >> /dev/null
RUN passwd -d root

RUN mkdir /root/.ssh

RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

EXPOSE 2222

CMD    ["/usr/sbin/sshd", "-D"]
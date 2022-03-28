FROM ubuntu:latest

RUN apt update -y && apt upgrade -y && apt install iputils-ping openssh-client openssh-server bash netcat iproute2 net-tools -y

ENTRYPOINT ["/bin/sh", "-c"]

CMD ["bash"]
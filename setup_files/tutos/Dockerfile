FROM ubuntu:latest

RUN apt update -y && apt upgrade -y && apt install iputils-ping openssh-client openssh-server bash netcat -y

ENTRYPOINT ["/bin/sh", "-c"]

CMD ["bash"]
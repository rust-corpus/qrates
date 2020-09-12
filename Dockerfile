FROM ubuntu:18.04
MAINTAINER Vytautas Astrauskas "vastrauskas@gmail.com"

ENV DEBIAN_FRONTEND noninteractive

ARG UID=1000
ARG GID=1000

# Install prerequisites.
RUN apt-get update && \
    apt-get dist-upgrade -y && \
    apt-get install -y \
 		software-properties-common unzip zip wget \
		curl gdebi-core locales python-dev python3-dev git\
		apt-transport-https \
		ca-certificates \
		curl \
		gnupg-agent \
		software-properties-common \
		tmux \
		fish \
		build-essential \
		pkg-config libssl-dev \
		python sudo && \
    apt-get clean

RUN echo "developer:x:${UID}:${GID}:Developer,,,:/home/developer:/usr/bin/fish" >> /etc/passwd && \
	echo "developer:x:${GID}:" >> /etc/group && \
	echo "developer ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/developer && \
	chmod 0440 /etc/sudoers.d/developer && \
	PASS=$(< /dev/urandom tr -dc _A-Z-a-z-0-9 | head -c${1:-32};echo;) && \
	echo "developer:${PASS}" | chpasswd && \
    mkdir -p /home/developer && \
    chown developer /home/developer

USER developer
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup && \
    sh /tmp/rustup -y
ENV PATH=${PATH}:/home/developer/.cargo/bin

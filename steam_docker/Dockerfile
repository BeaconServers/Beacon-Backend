FROM debian:buster
ARG DEBIAN_FRONTEND=noninteractive
RUN useradd -m steam \
      && apt-get update \ 
      && apt-get install wget -y \
      && dpkg --add-architecture i386 \
      && apt-get update \
      && apt-get install lib32gcc1 -y
RUN mkdir -p /steamcmd
RUN chown -R steam /steamcmd
WORKDIR /steamcmd
USER steam
ADD --chown=steam:steam https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz /steamcmd
RUN wget https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz \
  && tar xvf steamcmd_linux.tar.gz
EXPOSE 27015/udp
WORKDIR /home/steam/
CMD /home/steam/run.sh

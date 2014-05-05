FROM ubuntu:12.04

MAINTAINER Hanno Braun, hb@hannobraun.de

RUN apt-get update
RUN apt-get install -y nginx

ADD http /srv/http
ADD nginx/nginx.conf /etc/nginx/nginx.conf
ADD bin/vndf-game-service /opt/vndf-game-service
ADD upstart/vndf-game-service.conf /etc/init/vndf-game-service.conf
ADD upstart/nginx.conf /etc/init/vndf-game-service.conf

EXPOSE 80
EXPOSE 34481

CMD ["init"]

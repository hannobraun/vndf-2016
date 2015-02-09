FROM ubuntu:14.04

MAINTAINER Hanno Braun, mail@hannobraun.de

RUN apt-get update
RUN apt-get dist-upgrade -y
RUN apt-get install -y nginx

ADD http                           /srv/http
ADD nginx/nginx.conf               /etc/nginx/nginx.conf
ADD vndf-game-service              /opt/vndf-game-service
ADD upstart/vndf-game-service.conf /etc/init/vndf-game-service.conf
ADD upstart/nginx.conf             /etc/init/nginx.conf

EXPOSE 80
EXPOSE 34481

CMD ["/sbin/init"]

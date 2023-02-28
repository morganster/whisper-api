

# Whisper

This Project is an experiment to build a social network api writen in rust.

## Features
  - User registration, login [Done]
  - User Posting, Deleting, Editing [In Progress]
  - User Follows [Pending]
  - User Feed [Pending]
  - User Comments [Pending]

## Config

in order to run this api, it needs to have a mariadb running and the env vars been setup, the is an example in .env_local, just need to be renamed to .env and fill with the correct info of the machine.

## Minimum Supported Rust version

Whisper MSRV is 1.66.1.

## Setting a Mariadb with Docker
1. run mariadb in a docker `docker network create whisper-network  &&  docker run --detach --name  whisperdb --network whisper-network --env MARIADB_USER=whisper --env MARIADB_PASSWORD=whisper --env MARIADB_ROOT_PASSWORD=whisper  mariadb:latest`
2. access and connect `docker run -it --network whisper-network --rm mariadb mariadb -hwhisperdb -uwhisper -p`
3. get ip of the container `docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' whisperdb` to make a connection from ouside of the container
4. run the created image `docker start whisperdb`
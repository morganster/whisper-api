


## Mariadb
1. run mariadb in a docker `docker network create twister-network  &&  docker run --detach --name  twisterdb --network twister-network --env MARIADB_USER=twister --env MARIADB_PASSWORD=twister --env MARIADB_ROOT_PASSWORD=twister  mariadb:latest`
2. access and connect `docker run -it --network twister-network --rm mariadb mariadb -htwisterdb -utwister -p`
3. get ip of the container `docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' twisterdb` to make a connection from ouside of the container
4. run the created image `docker start twisterdb`
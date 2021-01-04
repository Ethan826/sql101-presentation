docker build . -t baseball
docker container rm baseball && docker build . -t baseball && docker run --name baseball -e POSTGRES_HOST_AUTH_METHOD=trust baseball

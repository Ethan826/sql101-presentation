docker stop baseball || true && docker rm baseball || true
docker build . -t baseball && docker run --name baseball -e POSTGRES_HOST_AUTH_METHOD=trust baseball

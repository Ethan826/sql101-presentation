FROM postgres:13.1-alpine
ENV POSTGRES_DB baseball
COPY dump.sql /docker-entrypoint-initdb.d/

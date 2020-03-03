FROM postgres:9.6

COPY init-user-db.sh /docker-entrypoint-initdb.d/init-user-db.sh

ENV POSTGRES_PASSWORD root
ENV POSTGRES_USER admin
ENV POSTGRES_DB actix-rentals

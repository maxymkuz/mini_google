FROM postgres:latest
ENV POSTGRES_PASSWORD=postgres
ENV POSTGRES_USER=admin
ENV POSTGRES_DB=database
COPY initializer.sql /docker-entrypoint-initdb.d/initializer.sql
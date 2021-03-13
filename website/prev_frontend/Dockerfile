FROM alpine:3.13

WORKDIR /usr/src
COPY zola-webpage .
EXPOSE 5000/tcp

# Install zola and run the site
RUN apk add zola --repository http://dl-cdn.alpinelinux.org/alpine/edge/community/
RUN zola build
CMD ["zola", "serve", "--interface", "0.0.0.0", "--port", "5000"]
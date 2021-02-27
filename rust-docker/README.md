A basic rust docker image.

To build an image and run it:
```bash
./build.sh

# To launch the helloworld app
docker run --rm lastgenius/rust-hello-world

# To launch the container in interactive mode with a shell
docker run --rm -it lastgenius/rust-hello-world sh
```

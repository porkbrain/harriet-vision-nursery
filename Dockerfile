FROM ubuntu:19.04

# Exposes http server APIs.
EXPOSE 8000

# Sets working directory where the binary is going to be copied to.
WORKDIR /usr/bin/harriet

# Copies the release binary.
COPY ./target/release/harriet-vision-nursery .

# Runs the application with http server keeping the main thread alive.
CMD ./harriet-vision-nursery

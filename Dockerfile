# Use a lightweight Linux base image with Rust installed
FROM rust:latest

# Install necessary tools
RUN apt-get update && apt-get install -y \
    build-essential \
    iptables \
    nftables \
    vim \
    && apt-get clean

# Set the working directory inside the container
WORKDIR /usr/src/logker

# Copy the current project files to the container
COPY . .

# Install project dependencies
RUN cargo build --release

# Set entrypoint for development
CMD ["bash"]

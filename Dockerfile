FROM rust:latest

USER root
ENV USER root

# Install package dependencies.
RUN apt-get update \
    && apt-get install -y \
    apt-utils \
    curl \
    gcc \ 
    software-properties-common \
    pkg-config \ 
    libssl-dev \ 
    cmake \
    postgresql-client \
    ffmpeg

# Customized install process for yt-dlp
RUN wget https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -P /bin
RUN chmod +x /bin/yt-dlp
 
# Copy source into container
WORKDIR /usr/src/rusty-monitor
COPY . .

# Build the application binary
RUN cargo build --release

# docker run "/usr/src/rusty-tube/target/release/rusty-tube" -- prod
#docker run --name epic_jackson --entrypoint /bin/bash rusty-tube
#docker run --name <CONTAINER_NAME> --entrypoint /bin/bash <IMAGE_NAME>
#docker build -t rusty-tube . 
#docker run -p 8000:8000 -it rusty-tube
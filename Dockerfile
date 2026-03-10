FROM rust:1.94-slim
RUN apt-get update && apt-get install -y \
    default-jre-headless \
    graphviz \
    gnuplot \
    wget \
    bzip2 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /opt
RUN wget https://github.com/jepsen-io/maelstrom/releases/download/v0.2.3/maelstrom.tar.bz2 \
    && tar -xf maelstrom.tar.bz2 \
    && rm maelstrom.tar.bz2
RUN chown -R root:root /opt/maelstrom \
    && chmod -R +rx /opt/maelstrom
ENV PATH="/opt/maelstrom:${PATH}"
WORKDIR /workspace
CMD ["bash"]

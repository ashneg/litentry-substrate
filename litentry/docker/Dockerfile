FROM ubuntu:18.04
ENV RUST_BACKTRACE 1
RUN apt-get update && \
        DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
        DEBIAN_FRONTEND=noninteractive apt-get install -y \
                libssl1.1 \
                ca-certificates \
                curl && \
# apt cleanup
        apt-get autoremove -y && \
        apt-get clean && \
        find /var/lib/apt/lists/ -type f -not -name lock -delete;

RUN mkdir -p /substrate/data
RUN mkdir -p /substrate/bin

# add substrate binary to docker image
COPY ./litentry /usr/local/bin/
COPY ./startup.sh /substrate/bin/

# check if executable works in this container
RUN /usr/local/bin/litentry --version

EXPOSE 30333 9933 9944

ENTRYPOINT ["/substrate/bin/startup.sh"]

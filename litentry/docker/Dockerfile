FROM ubuntu:16.04
RUN apt-get update
COPY litentry /root/
COPY start_up.sh /root/
RUN chmod+x /root/start_up.sh
ENTRYPOINT ["/root/start_up.sh"]


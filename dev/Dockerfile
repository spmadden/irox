FROM rust:latest
LABEL description="Irox development image, preconfigured with all required deps"
LABEL authors="spmadden"

COPY prep-environment.sh /
RUN /bin/bash /prep-environment.sh

ENTRYPOINT ["bash"]
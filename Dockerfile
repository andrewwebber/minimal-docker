FROM scratch
ENV LD_LIBRARY_PATH=/lib64
COPY ./libs/* /lib64/
COPY ./target/release/app ./app

ENTRYPOINT ["./app"]

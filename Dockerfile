FROM docker.io/ubuntu:24.04
ARG TARGETARCH

RUN mkdir -p /opt/app
WORKDIR /opt/app
COPY ./tools/target_arch.sh /opt/app
RUN --mount=type=bind,target=/context \
 cp /context/target/$(/opt/app/target_arch.sh)/release/backend /opt/app/{{project_name}}
CMD ["/opt/flower/backend"]
EXPOSE 3003

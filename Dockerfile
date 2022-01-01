FROM alpine

RUN apk add tini bash curl jq --no-cache
ENTRYPOINT [ "tini", "--" ]
WORKDIR /app
ENV USER=vault
ENV UID=1000
ENV GID=1000

RUN addgroup -S "$USER" && \ 
    adduser \
      --disabled-password \
      --gecos "" \
      --home "$(pwd)" \
      --ingroup "$USER" \
      --no-create-home \
      --uid "$UID" \
      "$USER"
RUN chmod 777 /usr/local/bin
RUN chmod -vR 777 $(pwd)

# kubectl
COPY install-kubectl-in-cluster.sh /usr/local/bin/install-kubectl-in-cluster.sh

COPY target/x86_64-unknown-linux-musl/release/kubeflow-api-auth /app/kubeflow-api-auth
COPY migrations /app/migrations
COPY frontend/dist /app/frontend/dist
COPY templates /app/templates
COPY .env /app/.env
COPY entrypoint.sh /entrypoint.sh

USER $USER

CMD [ "/entrypoint.sh" ]

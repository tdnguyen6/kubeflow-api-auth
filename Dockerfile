FROM alpine

RUN apk add tini --no-cache
ENTRYPOINT [ "tini" ]

WORKDIR /app

COPY target/x86_64-unknown-linux-musl/release/kubeflow-api-auth /app/kubeflow-api-auth
COPY target/migrations /app/migrations
COPY target/frontend/dist /app/frontend/dist
COPY target/templates /app/templates
COPY .env /app/.env

CMD [ "--", "/app/kubeflow-api-auth" ]

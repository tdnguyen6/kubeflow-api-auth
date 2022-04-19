(
  cd containers/rust-musl-builder
  docker-compose up
)
docker build . -t ghcr.io/tidunguyen/kubeflow-api-auth
docker push ghcr.io/tidunguyen/kubeflow-api-auth  

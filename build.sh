(
  cd containers/rust-musl-builder
  docker-compose up
)
docker build . -t cr.tidu.giize.com/kubeflow-api-auth
docker push cr.tidu.giize.com/kubeflow-api-auth  

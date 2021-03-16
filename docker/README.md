# Docker multi-arch builds
This is a stubborn way to build for multiple architectures using a single Dockerfile. 
Building requires `docker buildx`, see [Docker docs](https://docs.docker.com/buildx/working-with-buildx).

The latest docker images on DockerHub is built with the following command:

```docker buildx build --platform=linux/amd64,linux/arm/v7,linux/arm/v6,linux/arm64/v8 --tag danielalvsaaker/tf-viewer .```

Every platform separated by comma is built in parallell. Release building without build cache takes around 30 minutes on an Intel i5-4690K.

Note that this Dockerfile is meant to be run on x86_64 (linux/amd64), and may not work on a different architecture.

# contributing

[![Open in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/fruzitent/devpp)

## Prerequisites

- [cgroup2](https://rootlesscontaine.rs/getting-started/common/cgroup2/#enabling-cpu-cpuset-and-io-delegation)
- [containerd](https://docs.docker.com/engine/storage/containerd)
- [docker](https://docs.docker.com/engine/security/rootless/#prerequisites)
- [qemu](https://docs.docker.com/build/building/multi-platform/#strategies)

```shell
docker build --file "./contrib/containers/Containerfile" --platform "linux/amd64,linux/arm64" --progress "plain" --tag "ghcr.io/fruzitent/devpp:latest" --output "./out/" --target "artifacts" .
docker build --file "./contrib/containers/Containerfile" --platform "linux/amd64,linux/arm64" --progress "plain" --tag "ghcr.io/fruzitent/devpp:latest" .
```

# runlikers

Reverse-engineer `docker run` command line arguments from running containers.

Ever looked at a running container and wondered *"how did I start this thing?"*
`runlikers` inspects a container and prints the exact `docker run` command needed
to reproduce it — ports, volumes, env vars, labels, resource limits, and
everything else.

Inspired by [runlike](https://github.com/lavie/runlike) (Python), rewritten in
Rust for single-binary distribution and better performance.

## Features

- **Two data backends**: bollard API (default, with image-aware filtering) or
  `docker inspect` CLI (`--inspect`)
- **55+ supported options**: from `--volume` and `-p` to `--ulimit`,
  `--health-cmd`, `--sysctl`, and more
- **Pretty mode** (`-p`): multi-line output with `\` line continuations
- **Custom indent** (`--indent`): tab by default, or specify spaces
- **Tidy mode** (`--tidy`): filter out Docker daemon defaults
- **Dual format** (`--mount`): outputs both `--volume` and `--mount` formats
- **Remote hosts** (`-H`): `unix://`, `tcp://`, `ssh://`, `https://`
- **Pure Rust TLS**: via `rustls`, no OpenSSL dependency
- **Single static binary**, no runtime dependencies

## Installation

```bash
# From source
git clone git@github.com:dyrnq/runlikers.git
cd runlikers
cargo build --release
cp target/release/runlikers ~/.local/bin/

# Via cargo
cargo install --git git@github.com:dyrnq/runlikers.git

# Docker (works with bollard and --inspect modes)
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
    dyrnq/runlikers my-container

# Shell alias
alias runlikers='docker run --rm -v /var/run/docker.sock:/var/run/docker.sock dyrnq/runlikers'
```

## Usage

```bash
# Basic — inspect a running container
runlikers my-container

# Pretty-printed (multi-line, default: tab indent)
runlikers -p my-container

# 4-space indent in pretty mode
runlikers -p --indent '    ' my-container

# Exclude container name / labels
runlikers --no-name --no-labels my-container

# Use docker inspect CLI instead of bollard API
runlikers --inspect my-container

# Filter out daemon defaults
runlikers --tidy my-container

# Combine modes
runlikers --inspect --tidy -p my-container

# Emit --mount alongside --volume
runlikers --mount my-container

# Pipe docker inspect output directly
docker inspect my-container | runlikers --stdin

# Connect to a remote Docker host
runlikers -H tcp://192.168.1.100:2375 my-container
runlikers -H ssh://user@remote-host my-container
```

## Options

### CLI flags

| Flag | Description |
|---|---|
| `[CONTAINER]` | Container name or ID |
| `--no-name` | Omit `--name` from output |
| `--use-volume-id` | Keep the full volume ID instead of the short name |
| `-p, --pretty` | Multi-line output with `\` line continuations |
| `--indent` | Pretty indent string (default: tab, e.g. `'    '` for 4 spaces) |
| `-s, --stdin` | Read `docker inspect` JSON from stdin |
| `-l, --no-labels` | Omit `--label` from output |
| `--mount` | Also emit `--mount` format alongside `--volume` |
| `--inspect` | Use `docker inspect` CLI instead of bollard API |
| `--tidy` | Filter out Docker daemon defaults |
| `-H, --host` | Docker daemon address (`unix://`, `tcp://`, `ssh://`, `https://`) |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

### Supported docker run options

<details>
<summary>Click to expand (55+ options)</summary>

| Option | Docker API field |
|---|---|
| `--name` | `Name` |
| `-h, --hostname` | `Config.Hostname` |
| `--user` | `Config.User` |
| `--mac-address` | `Config.MacAddress` / `NetworkSettings.MacAddress` |
| `--ip` | `NetworkSettings.Networks.*.IPAMConfig.IPv4Address` |
| `--ip6` | `NetworkSettings.Networks.*.IPAMConfig.IPv6Address` |
| `--expose` | `NetworkSettings.Ports` (null binding) |
| `-p, --publish` | `NetworkSettings.Ports` / `HostConfig.PortBindings` |
| `-P` | `HostConfig.PublishAllPorts` |
| `-v, --volume` | `Mounts` / `HostConfig.Binds` |
| `--mount` | `Mounts` / `HostConfig.Binds` (via `--mount` flag) |
| `--tmpfs` | `Mounts` (type=tmpfs) |
| `--link` | `HostConfig.Links` |
| `--network` | `HostConfig.NetworkMode` |
| `--network-alias` | `NetworkSettings.Networks.*.Aliases` |
| `--link-local-ip` | `NetworkSettings.Networks.*.LinkLocalIPs` |
| `--dns` | `HostConfig.Dns` |
| `--dns-option` | `HostConfig.DnsOptions` |
| `--dns-search` | `HostConfig.DnsSearch` |
| `--add-host` | `HostConfig.ExtraHosts` |
| `-e, --env` | `Config.Env` (filtered against image) |
| `--entrypoint` | `Config.Entrypoint` (filtered against image) |
| `--workdir` | `Config.WorkingDir` |
| `-l, --label` | `Config.Labels` (filtered against image) |
| `--restart` | `HostConfig.RestartPolicy` |
| `--privileged` | `HostConfig.Privileged` |
| `--cap-add` | `HostConfig.CapAdd` |
| `--cap-drop` | `HostConfig.CapDrop` |
| `--device` | `HostConfig.Devices` |
| `--pid` | `HostConfig.PidMode` |
| `--ipc` | `HostConfig.IpcMode` |
| `--uts` | `HostConfig.UTSMode` |
| `--userns` | `HostConfig.UsernsMode` |
| `--security-opt` | `HostConfig.SecurityOpt` |
| `--sysctl` | `HostConfig.Sysctls` |
| `--group-add` | `HostConfig.GroupAdd` |
| `--runtime` | `HostConfig.Runtime` |
| `--init` | `HostConfig.Init` |
| `--rm` | `HostConfig.AutoRemove` |
| `--read-only` | `HostConfig.ReadonlyRootfs` |
| `--log-driver` / `--log-opt` | `HostConfig.LogConfig` |
| `-m, --memory` | `HostConfig.Memory` |
| `--memory-reservation` | `HostConfig.MemoryReservation` |
| `--memory-swap` | `HostConfig.MemorySwap` |
| `--memory-swappiness` | `HostConfig.MemorySwappiness` |
| `--kernel-memory` | `HostConfig.KernelMemory` |
| `--shm-size` | `HostConfig.ShmSize` |
| `--cpuset-cpus` / `--cpuset-mems` | `HostConfig.CpusetCpus` / `CpusetMems` |
| `--ulimit` | `HostConfig.Ulimits` |
| `--pids-limit` | `HostConfig.PidsLimit` |
| `--oom-kill-disable` | `HostConfig.OomKillDisable` |
| `--oom-score-adj` | `HostConfig.OomScoreAdj` |
| `--device-read-bps` / `--device-read-iops` | `HostConfig.BlkioDeviceReadBps` / `BlkioDeviceReadIOps` |
| `--device-write-bps` / `--device-write-iops` | `HostConfig.BlkioDeviceWriteBps` / `BlkioDeviceWriteIOps` |
| `--health-cmd` / `--health-interval` / etc. | `HostConfig.Healthcheck` |
| `--no-healthcheck` | `HostConfig.Healthcheck` (Test=`["NONE"]`) |
| `--stop-signal` | `Config.StopSignal` |
| `--stop-timeout` | `Config.StopTimeout` |
| `--storage-opt` | `HostConfig.StorageOpt` |
| `-d, --detach` | `Config.AttachStdout` |
| `-t` | `Config.Tty` |
| `--volumes-from` | `HostConfig.VolumesFrom` |

</details>

## Data backends

### bollard API (default)

Uses the [bollard](https://github.com/fussybeaver/bollard) Rust library to talk
to the Docker daemon via its REST API. Also fetches image metadata to filter
out options inherited from the image (e.g. `ENV` lines, image `LABEL`s, default
`ENTRYPOINT`).

```bash
runlikers my-container
```

### `--inspect` mode (docker inspect CLI)

Runs `docker inspect <container>` as a subprocess and parses its JSON.
No image metadata — everything from the container is shown verbatim.

```bash
runlikers --inspect my-container
```

### `--stdin` mode

Read raw `docker inspect` JSON from stdin — useful for scripting or offline
analysis where Docker is not available.

```bash
docker inspect my-container | runlikers --stdin
```

## Tidy mode

`--tidy` filters out options that are likely Docker daemon defaults rather than
user-specified:

| Filtered option | Reason |
|---|---|
| `--runtime=runc` | Default runtime |
| `--ipc=private` / `--ipc=shareable` | Default IPC mode |
| `--shm-size="67108864"` | Default 64 MB shared memory |
| `--stop-signal=<value>` | When it matches the image's `StopSignal` |

## Example

```bash
# Default output
$ runlikers -p nginx
docker run \
    --name=nginx \
    --hostname=debian \
    --volume /tmp/nginx-conf:/etc/nginx/conf.d:ro \
    --network=host \
    --restart=always \
    --log-opt max-size=100m \
    --runtime=runc \
    --shm-size="67108864" \
    --ipc=private \
    --stop-signal=SIGQUIT \
    --ulimit NOFILE=10000:10000 \
    --detach=true \
    nginx:1.27.0-alpine \
    nginx -g 'daemon off;'

# With --tidy (defaults filtered out)
$ runlikers --tidy -p nginx
docker run \
    --name=nginx \
    --hostname=debian \
    --volume /tmp/nginx-conf:/etc/nginx/conf.d:ro \
    --network=host \
    --restart=always \
    --log-opt max-size=100m \
    --ulimit NOFILE=10000:10000 \
    --detach=true \
    nginx:1.27.0-alpine \
    nginx -g 'daemon off;'
```

## Comparison with runlike (Python)

| Feature | runlike (Python) | runlikers (Rust) |
|---|---|---|
| Runtime | Python 3.8+ | Single static binary |
| Backend | `docker inspect` CLI | bollard API (default) + `--inspect` CLI |
| Image filtering | ✅ Env, labels, entrypoint | ✅ Same, plus `--stop-signal` |
| Defaults filtering | ❌ | ✅ `--tidy` |
| `--mount` format | ❌ | ✅ |
| Healthcheck | ❌ | ✅ |
| Ulimit / Blkio / Sysctl / etc. | ❌ | ✅ |
| Remote hosts | ❌ | ✅ `-H` flag |
| Pretty indent | Fixed tab | Customizable `--indent` |

## License

MIT

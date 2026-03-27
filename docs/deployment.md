# Deployment

## Service Form

This project is intended to be deployed as a containerized HTTP service.

## Container Files

The repository includes:

- `Dockerfile`
- `.dockerignore`

## Default Port

The current service listens on:

```text
8081
```

## Build Example

```bash
docker build -t re-indicators-calculation-service .
```

## Run Example

```bash
docker run --rm -p 8081:8081 re-indicators-calculation-service
```

## Runtime Configuration

The service supports these environment variables:

- `HEX_CORE_BASE_URL`
- `ARTIFACT_BASE_URL_TEMPLATE`
- `HTTP_TIMEOUT_SECS`
- `BIND_ADDRESS`
- `PORT`

Defaults:

```text
HEX_CORE_BASE_URL=http://127.0.0.1:8080
ARTIFACT_BASE_URL_TEMPLATE=https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v{version}/generated/
HTTP_TIMEOUT_SECS=15
BIND_ADDRESS=0.0.0.0
PORT=8081
```

Container example:

```bash
docker run --rm \
  -p 8081:8081 \
  -e HEX_CORE_BASE_URL=http://hex-core-service:8080 \
  -e ARTIFACT_BASE_URL_TEMPLATE=https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v{version}/generated/ \
  -e HTTP_TIMEOUT_SECS=15 \
  -e BIND_ADDRESS=0.0.0.0 \
  -e PORT=8081 \
  re-indicators-calculation-service
```

`ARTIFACT_BASE_URL_TEMPLATE` must contain `{version}` so the service can resolve the released artifact set for the selected model version.

## Local-Only Demonstrator

This repository also includes a local-only integration path documented in [Local Testing](local-testing.md).

That path is intended for manual checks on a developer machine and is not part of CI.

## Next Deployment Concerns

As the implementation grows, deployment documentation should be expanded to cover:

- runtime configuration for artifact resolution
- outbound connectivity to published model artifacts
- integration settings for `hex-core-service`
- health and readiness behavior

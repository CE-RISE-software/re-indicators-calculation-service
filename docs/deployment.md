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

## Next Deployment Concerns

As the implementation grows, deployment documentation should be expanded to cover:

- runtime configuration for artifact resolution
- outbound connectivity to published model artifacts
- integration settings for `hex-core-service`
- health and readiness behavior

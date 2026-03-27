# Local Testing

This repository includes a local-only demonstrator path for manual integration testing.

It is intended to verify the full local flow:

- run `hex-core-service` locally in memory mode
- run `re-indicators-calculation-service` locally
- validate an RE indicators payload through `hex-core-service`
- compute the RE indicators result through this service

This path is not part of CI.

## Files

The local testing setup is defined by:

- `demo.sh`
- `scripts/validate-local-compose.sh`
- `compose/docker-compose.yml`
- `compose/.env.example`
- `payloads/recycle_battery_compute_request.json`

## What It Uses

The local stack starts:

- `hex-core-service`
- `re-indicators-calculation-service`

Notes:

- `hex-core-service` runs with `IO_ADAPTER_ID=memory`
- `hex-core-service` runs with `AUTH_MODE=none`
- `hex-core-service` is pulled as a container image
- the registry catalog points directly to the published `schema.json`, `shacl.ttl`, and `model.ttl` artifact URLs on the published Pages site
- this service resolves `calculation.json` directly from the published Pages artifact URL
- no database is required
- this setup is only for local development and manual validation

## Commands

Run the local validation flow:

```bash
./demo.sh validate
```

Run the local demonstration flow:

```bash
./demo.sh demo
```

Stop the stack:

```bash
./demo.sh down
```

Remove the stack and local synced artifacts:

```bash
./demo.sh clean
```

## Expected Result

`./demo.sh validate` should complete with:

- local `hex-core-service` readiness confirmed
- local calculation service readiness confirmed
- delegated validation passing
- compute request returning `200`

The current sample payload is:

- model version `0.0.4`
- indicator `REcycle_Battery`

The current expected computed score is approximately:

```text
0.138572436192
```

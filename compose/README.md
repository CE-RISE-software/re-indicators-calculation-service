# Local Compose Stack

This directory contains a local-only demonstrator stack for the RE indicators calculation service.

The stack is intentionally separate from CI/CD:

- it is not run by the Forgejo CI workflow
- it is intended for manual local integration checks on a developer machine
- it runs `hex-core-service` in `memory` mode with `AUTH_MODE=none`

Services:

- `artifact-server`
- `hex-core-service`
- `re-indicators-calculation-service`

No database is included in this stack.

Before starting the stack, sync the required released RE indicators artifacts into `compose/registry/artifacts/` with:

```bash
./scripts/sync-local-artifacts.sh
```

The synced local artifact set currently includes:

- `schema.json`
- `shacl.ttl`
- `model.ttl`
- `calculation.json`

Then run the local demonstration with:

```bash
./demo.sh demo
```

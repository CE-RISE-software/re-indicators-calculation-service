# Local Compose Stack

This directory contains a local-only demonstrator stack for the RE indicators calculation service.

The stack is intentionally separate from CI/CD:

- it is not run by the Forgejo CI workflow
- it is intended for manual local integration checks on a developer machine
- it runs `hex-core-service` in `memory` mode with `AUTH_MODE=none`

Services:

- `hex-core-service`
- `re-indicators-calculation-service`

No database is included in this stack.

The local stack uses a registry catalog with direct published artifact URLs on Codeberg. The calculation service also resolves `calculation.json` directly from the published versioned URL template.

Then run the local demonstration with:

```bash
./demo.sh demo
```

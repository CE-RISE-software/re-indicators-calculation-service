# API Overview

## Design Goals

The API is designed around a simple synchronous service contract:

- callers submit a validated payload
- callers choose the RE indicators specification version
- the service returns structured validation and computation output

The API should stay explicit and domain-specific rather than mirroring the generic `{model}` pattern used by `hex-core-service`.

## Core Principles

- fixed model family: `re-indicators-specification`
- version is required input
- if version is omitted during current testing, the service defaults to `0.0.3`
- payload submission is the primary compute contract
- SHACL validation details are part of the response
- computation details are returned in a form that can be reused in downstream records or other models

## Artifact Source

The service is hardwired to the RE indicators generated publication lineage:

```text
https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{version}/generated/
```

The only variable is `{version}`.

## Current Endpoints

- `GET /health`
- `POST /compute`

## Request Shape

Current compute requests include:

```json
{
  "model_version": "0.0.3",
  "payload": {
    "id": "assessment-1",
    "indicator_specification_id": "REcycle_PV"
  }
}
```

## Response Shape

Current compute responses include:

- fixed model family
- selected model version
- resolved artifact base URL
- original payload
- validation summary
- computation result

The exact schema will continue to evolve as SHACL validation and scoring logic are implemented.

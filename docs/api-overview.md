# API Overview

## Design Goals

The API is designed around a simple synchronous service interaction:

- callers submit a validated payload
- callers choose the RE indicators specification version
- the service returns structured validation and computation output

The API should stay explicit and domain-specific rather than mirroring the generic `{model}` pattern used by `hex-core-service`.

## Core Principles

- fixed model family: `re-indicators-specification`
- version is required input
- if version is omitted during current testing, the service defaults to `0.0.4`
- payload submission is the primary compute input pattern
- validation is delegated to `hex-core-service` during computation
- computation details are returned in a form that can be reused in downstream records or other models

## Artifact Source

The service is hardwired to the RE indicators generated publication lineage:

```text
https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v{version}/generated/
```

The only variable is `{version}`.

## Current Endpoints

- `GET /health`
- `GET /openapi.json`
- `POST /compute`

## Request Shape

Current compute requests include:

```json
{
  "model_version": "0.0.4",
  "payload": {
    "indicator_specification_id": "REcycle_PV",
    "parameter_assessments": [
      {
        "parameter_id": "P1_product_diagnosis",
        "question_answers": [
          {
            "question_id": "Q1.1",
            "selected_answer_id": "product_id_all_key_info"
          }
        ]
      }
    ]
  }
}
```

## Response Shape

Current compute responses include:

- fixed model family
- selected model version
- resolved artifact base URL
- resolved artifact URLs for calculation, SHACL, schema, and OWL files
- original payload
- normalized validation summary from the `hex-core-service` validation interaction
- computation result

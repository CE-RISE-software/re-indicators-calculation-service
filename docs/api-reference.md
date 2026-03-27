# API Reference

## Base URL

```text
http://<host>:8081/
```

## `GET /health`

Service health and fixed service identity.

Example response:

```json
{
  "status": "ok",
  "model_family": "re-indicators-specification",
  "validation_basis": "shacl",
  "default_testing_version": "0.0.4",
  "artifact_base_url_template": "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v{version}/generated/",
  "hex_core_base_url": "http://127.0.0.1:8080",
  "http_timeout_secs": 15,
  "bind_address": "0.0.0.0",
  "port": 8081,
  "calculation_artifact_url_template": "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v{version}/generated/calculation.json"
}
```

## `GET /openapi.json`

Machine-readable OpenAPI document for the current service API.

This endpoint returns the generated OpenAPI description for:

- `GET /health`
- `GET /openapi.json`
- `POST /compute`

Example usage:

```bash
curl -sS http://127.0.0.1:8081/openapi.json
```

## `POST /compute`

Accept a validated RE indicators payload and compute a structured result for a selected model version.

If present, the incoming bearer token is forwarded to `hex-core-service` for delegated validation.

### Request Schema

Top-level fields:

- `model_version`
  - type: `string | null`
  - required: no
  - meaning: RE indicators model version to use
  - current default when omitted: `0.0.4`
- `payload`
  - type: object
  - required: yes
  - meaning: assessment payload used for delegated validation and computation

Payload fields currently used by computation:

- `payload.indicator_specification_id`
  - type: `string`
  - required: yes
- `payload.parameter_assessments`
  - type: array
  - required: no
  - default: `[]`
- `payload.parameter_assessments[].parameter_id`
  - type: `string`
  - required: yes
- `payload.parameter_assessments[].question_answers`
  - type: array
  - required: no
  - default: `[]`
- `payload.parameter_assessments[].question_answers[].question_id`
  - type: `string`
  - required: yes
- `payload.parameter_assessments[].question_answers[].selected_answer_id`
  - type: `string | null`
  - required: yes

### Request

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

### Response Schema

Top-level fields:

- `model_family`
  - type: `string`
  - fixed value: `re-indicators-specification`
- `model_version`
  - type: `string`
- `artifact_base_url`
  - type: `string`
  - meaning: resolved artifact publication base for the selected version
- `artifacts`
  - type: object
  - meaning: resolved URLs for released RE indicators artifacts
- `payload`
  - type: object
  - meaning: echoed assessment payload used for validation and computation
- `validation`
  - type: object
  - meaning: normalized delegated validation result
- `result`
  - type: object
  - meaning: computed score output

Artifact fields:

- `artifacts.model_version`
- `artifacts.base_url`
- `artifacts.calculation_url`
- `artifacts.shacl_url`
- `artifacts.schema_url`
- `artifacts.owl_url`

Validation fields:

- `validation.basis`
  - type: `string`
  - current value: `shacl`
- `validation.source`
  - type: `string`
  - current value: `hex-core-service`
- `validation.validation_url`
  - type: `string`
- `validation.status`
  - type: `string`
  - current values:
    - `validated_by_hex_core`
    - `validation_skipped`
- `validation.passed`
  - type: `boolean | null`
- `validation.finding_count`
  - type: `integer | null`
- `validation.findings_present`
  - type: `boolean | null`
- `validation.raw_report`
  - type: `object | null`
  - meaning: raw upstream validation report from `hex-core-service`
- `validation.details`
  - type: `string[]`

Computation fields:

- `result.status`
  - type: `string`
  - current value on success: `computed`
- `result.total_score`
  - type: `number | null`
- `result.parameter_scores`
  - type: array
- `result.parameter_scores[].parameter_id`
  - type: `string`
- `result.parameter_scores[].computed_score`
  - type: `number | null`
- `result.parameter_scores[].question_scores`
  - type: array
- `result.parameter_scores[].question_scores[].question_id`
  - type: `string`
- `result.parameter_scores[].question_scores[].selected_answer_id`
  - type: `string | null`
- `result.parameter_scores[].question_scores[].answer_score`
  - type: `number | null`
- `result.notes`
  - type: `string[]`

### Current Response

```json
{
  "model_family": "re-indicators-specification",
  "model_version": "0.0.4",
  "artifact_base_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v0.0.4/generated/",
  "artifacts": {
    "model_version": "0.0.4",
    "base_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v0.0.4/generated/",
    "calculation_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v0.0.4/generated/calculation.json",
    "shacl_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v0.0.4/generated/shacl.ttl",
    "schema_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v0.0.4/generated/schema.json",
    "owl_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/raw/tag/pages-v0.0.4/generated/owl.ttl"
  },
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
  },
  "validation": {
    "basis": "shacl",
    "source": "hex-core-service",
    "validation_url": "http://127.0.0.1:8080/models/re-indicators-specification/versions/0.0.4:validate",
    "status": "validated_by_hex_core",
    "passed": true,
    "finding_count": 0,
    "findings_present": false,
    "raw_report": {
      "passed": true,
      "results": []
    },
    "details": [
      "Payload validation was delegated to hex-core-service."
    ]
  },
  "result": {
    "status": "computed",
    "total_score": 0.8,
    "parameter_scores": [
      {
        "parameter_id": "P1_product_diagnosis",
        "computed_score": 3.0,
        "question_scores": [
          {
            "question_id": "Q1.1",
            "selected_answer_id": "product_id_all_key_info",
            "answer_score": 3.0
          }
        ]
      }
    ],
    "notes": [
      "Scores are computed from calculation.json question scores and parameter weights."
    ]
  }
}
```

## Notes

- the model family is fixed internally and is not a request parameter
- the service resolves artifacts from the configured `ARTIFACT_BASE_URL_TEMPLATE`
- `model_version` is the only version selector
- when omitted in the current implementation, `model_version` defaults to `0.0.4` for testing
- the current implementation delegates validation to `hex-core-service` using `POST /models/re-indicators-specification/versions/{version}:validate`
- computation uses the published `calculation.json` artifact for the same version
- the payload is returned so downstream systems can place computation output where needed
- the effective artifact base template, `hex-core-service` base URL, and HTTP timeout are runtime-configurable and exposed by `GET /health`
- the machine-readable OpenAPI document is exposed at `GET /openapi.json`
- the validation section is normalized by this service and only keeps the upstream report as `raw_report`

## Error Responses

The service returns structured error responses with HTTP status codes.

Example shape:

```json
{
  "code": "VALIDATION_FAILED",
  "message": "Delegated validation through hex-core-service did not pass.",
  "details": {}
}
```

Error fields:

- `code`
  - type: `string`
  - meaning: machine-readable error identifier
- `message`
  - type: `string`
  - meaning: human-readable summary
- `details`
  - type: `object | null`
  - meaning: extra structured context when available

Current error categories include:

- `400` invalid request body shape for the compute contract
- `401` delegated validation unauthorized in `hex-core-service`
- `404` requested model version or `calculation.json` artifact not found
- `422` delegated validation failed or unknown `indicator_specification_id`
- `502` delegated validation upstream error or calculation artifact fetch error
- `500` internal service configuration/runtime error

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
  "default_testing_version": "0.0.3",
  "artifact_base_url_template": "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{version}/generated/"
}
```

## `POST /compute`

Accept a validated RE indicators payload and compute a structured result for a selected model version.

### Request

```json
{
  "model_version": "0.0.3",
  "payload": {
    "id": "assessment-1",
    "indicator_specification_id": "REcycle_PV"
  }
}
```

### Current Response

```json
{
  "model_family": "re-indicators-specification",
  "model_version": "0.0.3",
  "artifact_base_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v0.0.3/generated/",
  "payload": {
    "id": "assessment-1",
    "indicator_specification_id": "REcycle_PV"
  },
  "validation": {
    "basis": "shacl",
    "artifact_base_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v0.0.3/generated/",
    "status": "not_implemented",
    "details": [
      "SHACL-backed validation will be implemented against published RE indicators artifacts."
    ]
  },
  "result": {
    "status": "not_implemented",
    "total_score": null,
    "parameter_scores": [],
    "notes": [
      "The scoring engine is not implemented yet; this response only establishes the API contract."
    ]
  }
}
```

## Notes

- the model family is fixed internally and is not a request parameter
- the service resolves artifacts only from `https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{version}/generated/`
- `model_version` is the only version selector
- when omitted in the current scaffold, `model_version` defaults to `0.0.3` for testing
- the payload is returned so downstream systems can place computation output where needed
- the current implementation is a scaffold and will be extended with real artifact loading, SHACL validation, and computation

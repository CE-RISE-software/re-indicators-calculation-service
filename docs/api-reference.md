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
  "artifact_base_url_template": "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{version}/generated/",
  "hex_core_base_url": "http://127.0.0.1:8080"
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
  "artifacts": {
    "model_version": "0.0.3",
    "base_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v0.0.3/generated/",
    "shacl_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v0.0.3/generated/shacl.ttl",
    "schema_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v0.0.3/generated/schema.json",
    "owl_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v0.0.3/generated/owl.ttl",
    "route_url": "https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v0.0.3/generated/route.json"
  },
  "payload": {
    "id": "assessment-1",
    "indicator_specification_id": "REcycle_PV"
  },
  "validation": {
    "basis": "shacl",
    "source": "hex-core-service",
    "validation_url": "http://127.0.0.1:8080/models/re-indicators-specification/versions/0.0.3:validate",
    "status": "validated_by_hex_core",
    "passed": true,
    "report": {
      "passed": true,
      "results": []
    },
    "details": [
      "Payload validation was delegated to hex-core-service."
    ]
  },
  "result": {
    "status": "not_implemented",
    "total_score": null,
    "parameter_scores": [],
    "notes": [
      "The scoring engine is not implemented yet."
    ]
  }
}
```

## Notes

- the model family is fixed internally and is not a request parameter
- the service resolves artifacts only from `https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{version}/generated/`
- `model_version` is the only version selector
- when omitted in the current implementation, `model_version` defaults to `0.0.3` for testing
- the current implementation delegates validation to `hex-core-service` using `POST /models/re-indicators-specification/versions/{version}:validate`
- the payload is returned so downstream systems can place computation output where needed
- the current implementation will be extended with real computation logic after delegated validation

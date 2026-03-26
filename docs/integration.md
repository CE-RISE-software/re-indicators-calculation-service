# Integration With Hex Core Service

## Intended Relationship

The RE indicators calculation service is not a replacement for `hex-core-service`.

The current intended split is:

- `hex-core-service`: generic validation, registry access, and record persistence
- RE indicators calculation service: RE-specific scoring and structured computation output

## Preferred Flow

1. A client or user-facing application obtains or assembles the relevant digital passport payload.
2. This calculation service calls `hex-core-service` to validate the payload against the RE indicators model version being used.
3. After delegated validation, this service computes the RE indicators result.
4. The calculation response is then reused wherever needed in downstream records or models.

## Supported Direction

The primary interaction pattern for this service is payload submission.

Record lookup through `hex-core-service` may be added as a secondary integration mode, but it is not the main API entry pattern.

## Registry Constraint

Even when integrating with `hex-core-service`, this service should resolve only:

- model family: `re-indicators-specification`
- version: caller-selected tag

## Validation Delegation

Validation is not intended to be reimplemented independently here.

Instead, this service calls:

```text
POST /models/re-indicators-specification/versions/{version}:validate
```

on `hex-core-service` during computation.

# Integration With Hex Core Service

## Intended Relationship

The RE indicators calculation service is not a replacement for `hex-core-service`.

The current intended split is:

- `hex-core-service`: generic validation, registry access, and record persistence
- RE indicators calculation service: RE-specific scoring and structured computation output

## Preferred Flow

1. A client or user-facing application obtains or assembles the relevant digital passport payload.
2. Validation and storage can occur through `hex-core-service`.
3. The validated payload is submitted to this calculation service.
4. The calculation response is then reused wherever needed in downstream records or models.

## Supported Direction

The primary contract for this service is payload submission.

Record lookup through `hex-core-service` may be added as a secondary integration mode, but it is not the main API contract.

## Registry Constraint

Even when integrating with `hex-core-service`, this service should resolve only:

- model family: `re-indicators-specification`
- version: caller-selected tag

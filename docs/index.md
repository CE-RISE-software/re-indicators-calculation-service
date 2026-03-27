# RE Indicators Calculation Service

This site documents the CE-RISE RE indicators calculation service.

The service is a dedicated HTTP application that computes RE indicator results from the `re-indicators-specification` model family. It is intentionally domain-specific: the model family is fixed, the only user-selectable model dimension is the version tag, and validation is expected to run against the published SHACL artifacts for that selected version.

## What This Service Does

- accepts a validated RE indicators payload together with a selected specification version
- delegates validation to `hex-core-service` against the RE indicators model during computation
- computes structured RE indicator results based on the model-defined scoring logic
- returns a detailed result object that preserves payload, validation details, and computed scores for downstream use

## Relationship With `hex-core-service`

This service complements `hex-core-service` rather than replacing it.

- `hex-core-service` remains the generic model-aware service for validation, persistence, and record operations
- this service is the RE-specific computation layer
- the preferred interaction pattern for this service is submission of a validated payload

## Documentation Structure

- [Architecture](architecture.md): service boundaries and internal responsibilities
- [API Overview](api-overview.md): API goals and request/response shape
- [API Reference](api-reference.md): endpoint-level documentation
- [Deployment](deployment.md): container-first service deployment
- [Local Testing](local-testing.md): local-only demonstrator and manual integration flow
- [Integration With Hex Core Service](integration.md): expected interaction patterns
- [Project Scope](scope.md): fixed project constraints and intended boundaries

## Current Status

The repository currently contains the initial Rust service implementation and the first HTTP endpoints:

- `GET /health`
- `GET /openapi.json`
- `POST /compute`

Artifact loading, SHACL-backed validation, and scoring logic are the next implementation steps.

---

Funded by the European Union under Grant Agreement No. 101092281 — CE-RISE.  
Views and opinions expressed are those of the author(s) only and do not necessarily reflect those of the European Union or the granting authority (HADEA).
Neither the European Union nor the granting authority can be held responsible for them.

<a href="https://ce-rise.eu/" target="_blank" rel="noopener noreferrer">
  <img src="images/CE-RISE_logo.png" alt="CE-RISE logo" width="200"/>
</a>

© 2026 CE-RISE consortium.  
Licensed under the [European Union Public Licence v1.2 (EUPL-1.2)](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12).  
Attribution: CE-RISE project (Grant Agreement No. 101092281) and the individual authors/partners as indicated.

<a href="https://www.nilu.com" target="_blank" rel="noopener noreferrer">
  <img src="https://nilu.no/wp-content/uploads/2023/12/nilu-logo-seagreen-rgb-300px.png" alt="NILU logo" height="20"/>
</a>

Developed by NILU (Riccardo Boero — ribo@nilu.no) within the CE-RISE project.

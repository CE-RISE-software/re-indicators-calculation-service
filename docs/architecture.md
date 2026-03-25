# Architecture

## Overview

The RE indicators calculation service is a dedicated, containerized HTTP service implemented in Rust.

Its responsibility is narrow:

- accept RE indicators computation requests
- resolve the selected RE indicators specification version
- validate against SHACL artifacts
- compute and return structured scoring results

## Core Service Boundary

The service is intentionally not model-generic.

- fixed model family: `re-indicators-specification`
- variable model dimension: version tag only
- validation basis: published SHACL artifacts

## Internal Layers

The implementation should evolve around these layers:

- HTTP API layer: request parsing, response formatting, status codes
- artifact resolution layer: load published artifacts for a selected version
- validation layer: execute SHACL validation against the selected artifact set
- computation layer: derive parameter and total scores from the model-driven logic
- mapping layer: produce a structured response suitable for downstream digital passport use

## Deployment Shape

The intended deployment unit is a single service container.

No CLI or SDK is a primary deliverable for this repository.

# Project Scope

## Fixed Constraints

The following constraints currently define the scope of this service:

- only the `re-indicators-specification` model family is in scope
- the only user-selectable model dimension is the version tag
- artifact resolution is hardwired to `https://codeberg.org/CE-RISE-models/re-indicators-specification/src/tag/pages-v{version}/generated/`
- SHACL artifacts remain the validation basis, but validation is delegated through `hex-core-service`
- payload submission is the main compute input pattern
- the primary deliverable is a containerized HTTP service
- the API must be documented on this published pages site

## Deliberate Non-Goals For Now

- a generic multi-model calculation platform
- a dedicated CLI-first workflow
- project-local SDK generation

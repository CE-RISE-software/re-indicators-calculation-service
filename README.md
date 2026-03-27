# CE-RISE RE Indicators Calculation Service

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.19256093.svg)](https://doi.org/10.5281/zenodo.19256093)

A containerized HTTP service for computing RE indicator results from the published `re-indicators-specification` model artifacts.

The service:

- accepts an RE indicators assessment payload
- delegates validation to `hex-core-service`
- loads scoring data from the published `calculation.json`
- returns a structured result with payload, validation summary, and scores

---

## How to use the service

This service is meant to sit beside `hex-core-service`.

Typical flow:

1. An RE indicators payload is submitted to this service.
2. This service delegates validation to `hex-core-service`.
3. This service loads scoring data from the published RE indicators artifacts.
4. The service returns a structured computation result.

For full API and deployment details, use the Pages documentation:

- [Documentation Home](https://ce-rise-software.codeberg.page/re-indicators-calculation-service/)
- [API Reference](https://ce-rise-software.codeberg.page/re-indicators-calculation-service/api-reference.html)
- [Deployment](https://ce-rise-software.codeberg.page/re-indicators-calculation-service/deployment.html)
- [Local Testing](https://ce-rise-software.codeberg.page/re-indicators-calculation-service/local-testing.html)

## Quick Start

The released container image name in the registry is:

- `re-indicators-calculation`

Example pull from the current CE-RISE registry namespace:

```bash
docker pull rg.fr-par.scw.cloud/ce-rise-software/re-indicators-calculation:latest
```

Run the service locally:

```bash
docker run --rm \
  -p 8081:8080 \
  -e HEX_CORE_BASE_URL=http://host.docker.internal:8080 \
  -e ARTIFACT_BASE_URL_TEMPLATE=https://ce-rise-models.codeberg.page/re-indicators-specification/generated/ \
  -e HTTP_TIMEOUT_SECS=15 \
  rg.fr-par.scw.cloud/ce-rise-software/re-indicators-calculation:latest
```

Check the service:

```bash
curl -sS http://127.0.0.1:8081/health
curl -sS http://127.0.0.1:8081/openapi.json
```

Main compute endpoint:

- `POST /compute`

Minimal request shape:

```json
{
  "model_version": "0.0.4",
  "payload": {
    "indicator_specification_id": "REcycle_Battery",
    "parameter_assessments": []
  }
}
```

## Local Demonstration

This repository includes a local-only demonstrator that:

- pulls and runs `hex-core-service`
- builds and runs `re-indicators-calculation-service`
- validates and computes a sample `REcycle_Battery` payload locally

Typical commands:

```bash
./demo.sh validate
./demo.sh demo
```

## License

Licensed under the [European Union Public Licence v1.2 (EUPL-1.2)](LICENSE).

## Contributing

This repository is maintained on [Codeberg](https://codeberg.org/CE-RISE-software/re-indicators-calculation-service) and mirrored to GitHub for release archival workflows.

---

<a href="https://europa.eu" target="_blank" rel="noopener noreferrer">
  <img src="https://ce-rise.eu/wp-content/uploads/2023/01/EN-Funded-by-the-EU-PANTONE-e1663585234561-1-1.png" alt="EU emblem" width="200"/>
</a>

Funded by the European Union under Grant Agreement No. 101092281 — CE-RISE.  
Views and opinions expressed are those of the author(s) only and do not necessarily reflect those of the European Union or the granting authority (HADEA).
Neither the European Union nor the granting authority can be held responsible for them.

© 2026 CE-RISE consortium.  
Licensed under the [European Union Public Licence v1.2 (EUPL-1.2)](LICENSE).  
Attribution: CE-RISE project (Grant Agreement No. 101092281) and the individual authors/partners as indicated.

<a href="https://www.nilu.com" target="_blank" rel="noopener noreferrer">
  <img src="https://nilu.no/wp-content/uploads/2023/12/nilu-logo-seagreen-rgb-300px.png" alt="NILU logo" height="20"/>
</a>

Developed by NILU (Riccardo Boero — ribo@nilu.no) within the CE-RISE project.

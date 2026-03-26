# CE-RISE RE Indicators Calculation Service

[![DOI](https://zenodo.org/badge/DOI/TOBEOBTAINED.svg)](https://doi.org/TOBEOBTAINED)

A repository to provide a template for a faster setup of open science and open source software projects within the CE-RISE project.

---

## What this repository contains
- Actions in `.forgejo/workflows` for Codeberg runners to
- Actions in `.github/workflows` for GitHub runners in mirror to tag releases and initiate archiving on Zenodo through the GitHub / Zenodo integration.
- This README should be expanded as needed. Sections `License`, `Contributing`, and the footer should be retained but updated as needed.
- Documentation in `docs` using [Book](https://book.deno.land/) and released through Codeberg action on each push at https://ce-rise-software.codeberg.page/template-software/

## Local Demonstration

This repository now includes a local-only demonstrator stack modeled after the CE-RISE local demonstrator approach.

It lives in:

- `compose/`
- `scripts/`
- `demo.sh`

It is intended for manual local integration checks only and is not part of CI.

Typical flow:

```bash
./demo.sh demo
```

That path:

- syncs the released RE indicators artifacts locally
- starts `artifact-server`
- starts `hex-core-service` in memory mode with local no-auth settings
- starts `re-indicators-calculation-service`
- validates and computes a sample `REcycle_Battery` payload locally


## License

Licensed under the [European Union Public Licence v1.2 (EUPL-1.2)](LICENSE).

## Contributing

This repository is maintained on [Codeberg](https://codeberg.org/CE-RISE-software/template-software) — the canonical source of truth. The GitHub repository is a read mirror used for release archival and Zenodo integration. Issues and pull requests should be opened on Codeberg.

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

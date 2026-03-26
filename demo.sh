#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
compose_file="$repo_root/compose/docker-compose.yml"
compose_env="$repo_root/compose/.env"
compute_request="$repo_root/payloads/recycle_battery_compute_request.json"

ensure_env() {
  if [[ ! -f "$compose_env" ]]; then
    cp "$repo_root/compose/.env.example" "$compose_env"
  fi
}

compose_cmd() {
  docker compose -f "$compose_file" --env-file "$compose_env" "$@"
}

wait_for_http_code() {
  local url="$1"
  local expected="$2"
  local timeout="$3"
  local started code

  started="$(date +%s)"
  while true; do
    code="$(curl -sS -o /tmp/re-indicators-demo-body.json -w '%{http_code}' "$url" || true)"
    if [[ "$code" == "$expected" ]]; then
      return 0
    fi
    if (( "$(date +%s)" - started >= timeout )); then
      echo "Timed out waiting for $url to return HTTP $expected; last HTTP $code" >&2
      if [[ -f /tmp/re-indicators-demo-body.json ]]; then
        cat /tmp/re-indicators-demo-body.json >&2
      fi
      return 1
    fi
    sleep 2
  done
}

print_help() {
  cat <<'EOF'
Usage: ./demo.sh {up|demo|down|clean|validate|sync-artifacts}

Actions:
  up              Sync local artifacts and start the stack in the background.
  demo            Start the stack and run the local validation + compute demonstration.
  down            Stop the stack and remove compose-managed containers.
  clean           Stop the stack and remove compose-managed containers plus generated artifacts.
  validate        Run the local compose smoke validation.
  sync-artifacts  Download released RE indicators artifacts for local serving.
EOF
}

run_demo_pipeline() {
  echo
  echo "== Step 1: Wait for local stack =="
  wait_for_http_code "http://127.0.0.1:8080/admin/ready" "200" "120"
  wait_for_http_code "http://127.0.0.1:8081/health" "200" "120"
  echo "hex-core-service and re-indicators-calculation-service are healthy and ready."

  echo
  echo "== Step 2: Validate the RE indicators payload directly with hex-core-service =="
  jq '{payload: .payload}' "$compute_request" > /tmp/re-indicators-demo-validate.request.json
  status="$(curl -sS -o /tmp/re-indicators-demo-validate.response.json -w '%{http_code}' \
    -X POST http://127.0.0.1:8080/models/re-indicators-specification/versions/0.0.4:validate \
    -H "Content-Type: application/json" \
    --data-binary @/tmp/re-indicators-demo-validate.request.json)"
  echo "HTTP status: $status"
  jq '{passed, results}' /tmp/re-indicators-demo-validate.response.json
  [[ "$status" == "200" ]] || return 1

  echo
  echo "== Step 3: Compute the RE indicators result through the calculation service =="
  status="$(curl -sS -o /tmp/re-indicators-demo-compute.response.json -w '%{http_code}' \
    -X POST http://127.0.0.1:8081/compute \
    -H "Content-Type: application/json" \
    --data-binary @"$compute_request")"
  echo "HTTP status: $status"
  [[ "$status" == "200" ]] || {
    cat /tmp/re-indicators-demo-compute.response.json >&2
    return 1
  }
  jq '{model_version, validation: .validation.passed, total_score: .result.total_score}' /tmp/re-indicators-demo-compute.response.json

  echo
  echo "== Success Summary =="
  echo "Local validation passed through hex-core-service."
  echo "Local computation passed through re-indicators-calculation-service."
}

case "${1:-}" in
  up)
    ensure_env
    "$repo_root/scripts/sync-local-artifacts.sh"
    compose_cmd up -d artifact-server hex-core-service re-indicators-calculation-service
    ;;
  demo)
    ensure_env
    "$repo_root/scripts/sync-local-artifacts.sh"
    cleanup_demo() {
      docker compose -f "$compose_file" --env-file "$compose_env" down --remove-orphans >/tmp/re-indicators-demo-down.log 2>&1 || {
        cat /tmp/re-indicators-demo-down.log >&2
      }
    }
    trap cleanup_demo EXIT
    docker compose -f "$compose_file" --env-file "$compose_env" down --remove-orphans >/dev/null 2>&1 || true
    timeout 300s docker compose -f "$compose_file" --env-file "$compose_env" up -d artifact-server hex-core-service re-indicators-calculation-service >/tmp/re-indicators-demo-up.log 2>&1 || {
      cat /tmp/re-indicators-demo-up.log >&2
      exit 1
    }
    run_demo_pipeline
    ;;
  down)
    ensure_env
    compose_cmd down --remove-orphans
    ;;
  clean)
    ensure_env
    compose_cmd down --remove-orphans --volumes
    rm -rf "$repo_root/compose/registry/artifacts/re-indicators-specification"
    ;;
  validate)
    ensure_env
    "$repo_root/scripts/validate-local-compose.sh"
    ;;
  sync-artifacts)
    ensure_env
    "$repo_root/scripts/sync-local-artifacts.sh"
    ;;
  *)
    print_help >&2
    exit 1
    ;;
esac

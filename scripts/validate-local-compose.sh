#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
compose_file="$repo_root/compose/docker-compose.yml"
compose_env="$repo_root/compose/.env"
compose_env_example="$repo_root/compose/.env.example"
compute_request="$repo_root/payloads/recycle_battery_compute_request.json"

cp "$compose_env_example" "$compose_env"

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
    code="$(curl -sS -o /tmp/re-indicators-local-body.json -w '%{http_code}' "$url" || true)"
    if [[ "$code" == "$expected" ]]; then
      return 0
    fi
    if (( "$(date +%s)" - started >= timeout )); then
      echo "Timed out waiting for $url to return HTTP $expected; last HTTP $code" >&2
      if [[ -f /tmp/re-indicators-local-body.json ]]; then
        cat /tmp/re-indicators-local-body.json >&2
      fi
      return 1
    fi
    sleep 2
  done
}

cleanup() {
  timeout 180s docker compose -f "$compose_file" --env-file "$compose_env" down --remove-orphans >/dev/null 2>&1 || true
}

trap cleanup EXIT

echo "Check 1/5: Compose file renders"
compose_cmd config >/dev/null

echo "Check 2/5: Previous stack is cleared"
cleanup

echo "Check 3/5: Stack start is requested"
timeout 300s docker compose -f "$compose_file" --env-file "$compose_env" up -d --build hex-core-service re-indicators-calculation-service >/tmp/re-indicators-local-up.log 2>&1 || {
  cat /tmp/re-indicators-local-up.log >&2
  exit 1
}

echo "Check 4/5: Core and calculation service are ready"
wait_for_http_code "http://127.0.0.1:8080/admin/ready" "200" "120"
wait_for_http_code "http://127.0.0.1:8081/health" "200" "120"

echo "Check 5/5: Compute request succeeds against local stack"
status="$(curl -sS -o /tmp/re-indicators-local-compute.json -w '%{http_code}' \
  -X POST http://127.0.0.1:8081/compute \
  -H "Content-Type: application/json" \
  --data-binary @"$compute_request")"

echo "HTTP status: $status"
[[ "$status" == "200" ]] || {
  cat /tmp/re-indicators-local-compute.json >&2
  exit 1
}

jq '{model_version, validation: .validation.passed, total_score: .result.total_score}' /tmp/re-indicators-local-compute.json
echo "Validation passed"

#!/usr/bin/env bash
# Generate a Rust HTTP client from the Notes API OpenAPI spec using progenitor.
# Usage: ./generate-client.sh
# Prerequisites: cargo-progenitor, jq (cargo install cargo-progenitor; brew install jq)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
OPENAPI_JSON="$SCRIPT_DIR/openapi.json"
OPENAPI_30_JSON="$SCRIPT_DIR/openapi30.json"
CLIENT_DIR="$SCRIPT_DIR/client"

# 1. Emit OpenAPI spec (run dump-openapi from workspace root)
cd "$REPO_ROOT"
cargo run -p demo-notes --bin dump-openapi > "$OPENAPI_JSON"
echo "Wrote OpenAPI spec to $OPENAPI_JSON"

# 2. Convert 3.1 → 3.0 for progenitor (openapiv3 only supports 3.0.x)
if ! command -v jq &>/dev/null; then
  echo "jq not found. Install with: brew install jq (or your package manager)"
  exit 1
fi
jq '
  .openapi = "3.0.3"
  | def walk(f): if type == "object" then map_values(walk(f)) | f elif type == "array" then map(walk(f)) | f else f end;
  walk(
    if type == "object" and has("type") and (.type | type == "array") then
      .nullable = (.type | index("null") != null)
      | .type = (.type | map(select(. != "null")) | .[0])
    else . end
  )
' "$OPENAPI_JSON" > "$OPENAPI_30_JSON"
echo "Wrote OpenAPI 3.0 spec to $OPENAPI_30_JSON"

# 3. Generate client with progenitor
if ! command -v cargo-progenitor &>/dev/null; then
  echo "cargo-progenitor not found. Install with: cargo install cargo-progenitor"
  exit 1
fi

mkdir -p "$CLIENT_DIR"
cargo progenitor -i "$OPENAPI_30_JSON" -o "$CLIENT_DIR" -n demo_notes_client -v 0.1.0

echo "Client generated in $CLIENT_DIR"

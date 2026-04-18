#!/usr/bin/env bash
# Refresh the vendored LiteLLM pricing snapshot.
# Fetches the latest model_prices_and_context_window.json, filters to providers
# ccguilt tracks (Anthropic, Z.ai GLM, DeepSeek), and writes the slimmed-down
# result to vendor/litellm_prices.json.
#
# Run this whenever LiteLLM updates upstream pricing and you want ccguilt to
# reflect the new numbers in the next release.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="$REPO_ROOT/vendor/litellm_prices.json"
SRC="https://raw.githubusercontent.com/BerriAI/litellm/main/model_prices_and_context_window.json"

mkdir -p "$(dirname "$OUT")"
TMP=$(mktemp)
trap 'rm -f "$TMP"' EXIT

echo "Fetching $SRC..."
curl -sL "$SRC" -o "$TMP"

echo "Filtering to tracked providers..."
python3 - "$TMP" "$OUT" <<'PY'
import json, sys
src, dst = sys.argv[1], sys.argv[2]
with open(src) as f:
    raw = json.load(f)

def keep(name, entry):
    if not isinstance(entry, dict):
        return False
    provider = entry.get('litellm_provider', '').lower()
    if provider in ('anthropic', 'bedrock', 'vertex_ai-anthropic_models', 'anthropic_text'):
        return True
    lower = name.lower()
    return any(kw in lower for kw in ['claude', 'anthropic', 'glm-4', 'glm-5', 'glm4', 'glm5', 'deepseek'])

filtered = {k: v for k, v in raw.items() if keep(k, v)}
with open(dst, 'w') as f:
    json.dump(filtered, f, indent=2)
print(f"  kept {len(filtered)} of {len(raw)} entries")
PY

echo "Wrote $OUT"
ls -lh "$OUT"

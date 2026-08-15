#!/usr/bin/env bash
# Push credentials from .env into k8s Secrets under opaque names.
#
# Values never touch stdout and .env is never copied into a pod — containers read
# these through secretKeyRef in the pod spec.
#
# Usage:  scripts/make_cluster_secrets.sh [--dry-run]

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="$REPO_ROOT/.env"
KUBECTL="${KUBECTL:-kubectl}"
export KUBECONFIG="${KUBECONFIG:-$HOME/.kube/boogiebonjour}"
NS="${NS:-default}"

# Opaque names — see docs/gpu-and-runs.md#naming-rules.
# de2-datagen-* already exists and belongs to someone else; do not touch it.
SECRET_HF="de2-rl-test-hf"
SECRET_WANDB="de2-rl-test-wandb"

DRY_RUN=""
[[ "${1:-}" == "--dry-run" ]] && DRY_RUN="--dry-run=client"

[[ -f "$ENV_FILE" ]] || { echo "missing $ENV_FILE — copy .env.example and fill it in" >&2; exit 1; }

# shellcheck disable=SC1090
set -a; source "$ENV_FILE"; set +a

missing=()
for v in HF_TOKEN HF_CKPT_REPO WANDB_API_KEY WANDB_PROJECT; do
    [[ -n "${!v:-}" ]] || missing+=("$v")
done
if (( ${#missing[@]} )); then
    echo "unset in .env: ${missing[*]}" >&2
    exit 1
fi

case "$HF_CKPT_REPO" in
    */*) ;;
    *) echo "HF_CKPT_REPO must be <owner>/<repo>, got: $HF_CKPT_REPO" >&2; exit 1 ;;
esac

# The naming rule applies to k8s OBJECT names, not to personal-account paths:
# pod/job/PVC names show up in `kubectl get pods -A`, which everyone reads
# constantly, whereas a secret's value takes a deliberate `-o yaml` to see.
# The secret names below are hardcoded and opaque, so that invariant holds by
# construction. HF repo paths and W&B project names live in a personal account
# and may be descriptive — but they do land in a Secret, so say so out loud
# rather than silently.
# ${var,,} is bash 4+; macOS ships bash 3.2, so lowercase via tr.
_names="$(printf '%s %s' "$HF_CKPT_REPO" "$WANDB_PROJECT" | tr '[:upper:]' '[:lower:]')"
for word in spec verus rmm cca; do
    case "$_names" in
        *"$word"*)
            echo "note: '$word' appears in HF_CKPT_REPO/WANDB_PROJECT, which are stored" >&2
            echo "      in a k8s Secret. Readable by cluster-admins via 'kubectl get" >&2
            echo "      secret -o yaml'. Object names stay opaque. (docs/gpu-and-runs.md)" >&2
            break ;;
    esac
done

echo "cluster   : $(basename "$KUBECONFIG")  ns=$NS"
echo "secrets   : $SECRET_HF, $SECRET_WANDB"
echo "ckpt repo : $HF_CKPT_REPO"
echo "wandb     : ${WANDB_ENTITY:-<default entity>} / $WANDB_PROJECT"
echo

# `create --dry-run -o yaml | apply` makes this idempotent: re-running rotates the
# value in place instead of failing on AlreadyExists.
apply_secret() {
    local name="$1"; shift
    if [[ -n "$DRY_RUN" ]]; then
        local keys=""
        for kv in "$@"; do
            kv="${kv#--from-literal=}"   # strip the flag before splitting on '='
            keys="$keys ${kv%%=*}"
        done
        echo "  would apply secret/$name with keys:$keys"
        return
    fi
    "$KUBECTL" create secret generic "$name" -n "$NS" "$@" \
        --dry-run=client -o yaml | "$KUBECTL" apply -f - >/dev/null
    echo "  applied secret/$name"
}

apply_secret "$SECRET_HF" \
    --from-literal=token="$HF_TOKEN" \
    --from-literal=ckpt_repo="$HF_CKPT_REPO"

apply_secret "$SECRET_WANDB" \
    --from-literal=token="$WANDB_API_KEY" \
    --from-literal=project="$WANDB_PROJECT" \
    --from-literal=entity="${WANDB_ENTITY:-}"

echo
echo "done. Reference them in a pod spec as:"
cat <<YAML
        - {name: HF_TOKEN,      valueFrom: {secretKeyRef: {name: $SECRET_HF,    key: token}}}
        - {name: HF_CKPT_REPO,  valueFrom: {secretKeyRef: {name: $SECRET_HF,    key: ckpt_repo}}}
        - {name: WANDB_API_KEY, valueFrom: {secretKeyRef: {name: $SECRET_WANDB, key: token}}}
        - {name: WANDB_PROJECT, valueFrom: {secretKeyRef: {name: $SECRET_WANDB, key: project}}}
        - {name: WANDB_ENTITY,  valueFrom: {secretKeyRef: {name: $SECRET_WANDB, key: entity}}}
YAML

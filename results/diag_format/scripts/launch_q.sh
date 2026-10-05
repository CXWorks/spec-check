#!/usr/bin/env bash
# launch_q.sh <tag> <queue_gen args...>: start a queue fully detached, return at once.
tag=$1; shift
setsid nohup ~/spec-check/.venv/bin/python ~/diag/queue_gen.py --tag "$tag" "$@" \
  > ~/diag/logs/queue-$tag.log 2>&1 < /dev/null &
echo "queued $tag pid=$!"

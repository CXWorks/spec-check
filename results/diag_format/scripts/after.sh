#!/usr/bin/env bash
# after.sh <tag-to-wait-for> <tag> <queue_gen args...>: start a queue once another
# queue has logged QUEUE_DONE (log-based, so it cannot match its own process).
wait_tag=$1; shift
until grep -q "^QUEUE_DONE $wait_tag " ~/diag/logs/queue-$wait_tag.log 2>/dev/null; do sleep 30; done
exec ~/diag/launch_q.sh "$@"

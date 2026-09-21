# How the 36 runs were driven

The launcher scripts and per-run logs from jisenli-dev-0, kept so the runs can
be reproduced without reconstructing the orchestration from memory.

    run_phase1.sh   PSCI, 6 runs (2 models x 3 arms)
    run_docs.sh     SDEI + DRTM + FF-A, 18 runs
    watchdog.sh     waited for run_docs.sh, then queued SCMI + SBI
    run12.sh        SCMI + SBI, 12 runs, with a git pull and a precheck
    watchdog.log    launch timeline
    gen_docs/logs   per-run stdout, 30 runs
    gen_psci/logs   per-run stdout, 6 runs

Each run is one GPU; eight at a time, because two 9B processes on one card OOM.

The generated specs themselves are one level up in results/six_docs/.

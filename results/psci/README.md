# ARM PSCI (DEN0022F.b) — zero-shot, no gold

The RMM-trained checkpoints applied to a document they have never seen. PSCI has
no gold Verus specs, so there is no agreement axis: the measurable things are
whether Verus accepts the spec and what the spec says about itself.

Layout
  <arm>/<run>-<ckpt>/<version>/<command>.rs   the generated specs
  sweep-<arm>.json                            scripts/psci_sweep.py output

Every `sweep-*.json` carries the detector's own self-test under `self_test` and
`detector_sound`. A findings count from a run whose detector failed its fixtures
is not interpretable; all runs recorded here passed 6/6.

The in-pod sweeps that shipped inside the first artifacts were produced before
the linker-collision fix in psci_sweep.py and undercount by ~4 commands. The
`sweep-*.json` files at this level are the corrected re-runs, made locally on the
same Verus build (0.2026.04.12.f1166c4).

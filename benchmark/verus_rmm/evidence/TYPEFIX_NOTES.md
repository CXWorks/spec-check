# alp14 type fixes applied to make SCOPE reason-mode output type-check
#
# Scope: benchmark/alp14_focused.rs = preamble + the 13 spec fns carrying proof
# obligations + the 13 proof fns. Built from benchmark/alp14_reason.orig.rs
# (pristine `scope --target alp14 --mode reason` output).
#
# SCOPE ships eac5.patch / rel0.patch for exactly this purpose; no alp14.patch
# exists, so these are ours. All three fixes are recorded here so a reviewer can
# confirm no condition was weakened.
#
# 1. `int << nat` -> `* pow2(...)`   (2 sites, in rmi_vdev_map_spec / rmi_vdev_unmap_spec)
#    Verus has no spec_shl on the mathematical `int` type. pow2 is already
#    imported in the preamble (`use vstd::arithmetic::power2::pow2;`) and used
#    elsewhere in the same file. x << 10 == x * pow2(10); semantics preserved.
#
# 2. `^struct` -> `^pub struct`      (69 sites, preamble only)
#    Required for field accesses inside `pub open spec fn` to compile in a
#    standalone crate. Identical to the fix in
#    training/inconsistency_analysis_rmm.py::read_preamble (STATUS.md lesson 5).
#    No semantic effect.
#
# 3. NOTHING WAS COMMENTED OUT.
#    The whole-file attempt (benchmark/alp14_reason.wholefile.rs) needed ~100
#    clauses commented, all in commands with no proof obligation. The focused
#    file makes that unnecessary: none of the untranslated-prose or `::`
#    bit-concat artifacts occur in the 13 relevant commands.
#
# Verified: no fix touches any of the 13 proof-obligation commands' conditions.
# Result: `14 verified, 6 errors` (10 assertion failures).

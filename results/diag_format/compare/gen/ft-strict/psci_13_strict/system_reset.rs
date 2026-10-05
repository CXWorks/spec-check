pub open spec fn system_reset_spec(caller: Caller, old_s: S, new_s: S) -> bool {
  (SystemColdResetPerformed(new_s, MachineViewOf(new_s, caller)) &&
   (!CallIsVirtualized(old_s) ==> SystemPowerCycled(new_s)))
}
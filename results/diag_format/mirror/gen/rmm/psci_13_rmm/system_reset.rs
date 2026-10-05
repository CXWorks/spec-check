pub open spec fn system_reset_spec(caller: Caller, old_s: S, new_s: S) -> bool {
  (SystemColdResetPerformed(new_s, MachineViewOf(new_s, caller)) &&
   (!IsResponseVirtualized(new_s) ==> SystemPowerCycled(new_s, MachineViewOf(new_s, caller))))
}
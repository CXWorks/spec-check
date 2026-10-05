pub open spec fn system_reset_spec(result: (), old_s: S, new_s: S) -> bool {
    (SystemColdResetPerformed(MachineViewOf(caller)) && (!CallIsVirtualized() ==> SystemPowerCycled()))
}
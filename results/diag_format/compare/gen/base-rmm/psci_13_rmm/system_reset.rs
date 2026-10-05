pub open spec fn system_reset_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!IsResponseVirtualized(old_s) ==> SystemPowerCycled(MachineViewOf(caller)))
    && (SystemColdResetPerformed(MachineViewOf(caller)))
}
pub open spec fn psci_cpu_on_spec(
    fid: UInt64,
    target_cpu: Bits64,
    entry_point_address: Address,
    context_id: UInt32,
    result: PsciReturnCode,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsProtected(old_s, entry_point_address, CurrentRealm(old_s))
        ==> result == PSCI_INVALID_ADDRESS)
    && (!MpidrIsUsed(old_s, target_cpu)
        ==> result == PSCI_INVALID_PARAMETERS)
    && (RecFromMpidr(old_s, target_cpu).flags.runnable == RUNNABLE
        ==> result == PSCI_ALREADY_ON)
    && ((AddrIsProtected(old_s, entry_point_address, CurrentRealm(old_s))
        && MpidrIsUsed(old_s, target_cpu)
        && RecFromMpidr(old_s, target_cpu).flags.runnable != RUNNABLE)
        ==> (RecFromMpidr(new_s, target_cpu).pc == ToBits64(entry_point_address as int)
            && RecFromMpidr(new_s, target_cpu).flags.runnable == RUNNABLE))
}
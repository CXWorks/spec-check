pub open spec fn psci_cpu_on_spec(
    result: PsciReturnCode,
    target_cpu: u64,
    entry_point_address: Address,
    context_id: u32,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = CurrentRealm(old_s);
    let target_rec = RecFromMpidr(old_s, target_cpu);
    (!AddrIsProtected(old_s, entry_point_address, realm) ==> result == PSCI_INVALID_ADDRESS) && (
    !MpidrIsUsed(old_s, target_cpu) ==> result == PSCI_INVALID_PARAMETERS) && (
    target_rec.flags.runnable == RUNNABLE(old_s) ==> result == PSCI_ALREADY_ON) && (AddrIsProtected(
        old_s,
        entry_point_address,
        realm,
    ) && MpidrIsUsed(old_s, target_cpu) && target_rec.flags.runnable != RUNNABLE(old_s) ==> (result
        == PSCI_SUCCESS && new_s.recs[target_rec].pc == ToBits64(UInt(old_s, entry_point_address))
        && new_s.recs[target_rec].flags.runnable == RUNNABLE(new_s)))
}
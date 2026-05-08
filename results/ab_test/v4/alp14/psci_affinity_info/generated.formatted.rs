pub open spec fn PSCI_AFFINITY_INFO_spec(
    s: S,
    target_affinity: Bits64,
    lowest_affinity_level: UInt32,
    result: PsciReturnCode,
) -> bool {
    let target_rec = RecFromMpidr(s, target_affinity);
    ((lowest_affinity_level != 0 ==> result == PSCI_INVALID_PARAMETERS) && (!MpidrIsUsed(
        s,
        target_affinity,
    ) ==> result == PSCI_INVALID_PARAMETERS) && ((target_rec.flags.runnable == RUNNABLE
        && lowest_affinity_level == 0 && MpidrIsUsed(s, target_affinity)) ==> result
        == PSCI_SUCCESS) && ((target_rec.flags.runnable == NOT_RUNNABLE && lowest_affinity_level
        == 0 && MpidrIsUsed(s, target_affinity)) ==> result == PSCI_OFF))
}
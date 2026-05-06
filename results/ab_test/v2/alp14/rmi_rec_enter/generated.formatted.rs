pub open spec fn rmi_rec_enter_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    run_ptr: Address,
    rec_ptr: Address,
) -> bool {
    // Failure conditions - input validation
    (!AddrIsGranuleAligned(run_ptr) ==> result == RMI_ERROR_INPUT) && (!GranuleAccessPermitted(
        run_ptr,
        PAS_NS,
    ) ==> result == RMI_ERROR_INPUT) && (!AddrIsGranuleAligned(rec_ptr) ==> result
        == RMI_ERROR_INPUT) && (!PaIsDelegable(rec_ptr) ==> result == RMI_ERROR_INPUT) && (
    GranuleAt(old_s, rec_ptr).state != REC ==> result == RMI_ERROR_INPUT)
        &&
    // Failure conditions - realm state
    (RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_NEW ==> result == RMI_ERROR_REALM)
        && (RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state == REALM_SYSTEM_OFF ==> result
        == RMI_ERROR_REALM) &&
    // Failure conditions - REC state
    (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> result == RMI_ERROR_REC) && (RecAt(
        old_s,
        rec_ptr,
    ).flags.runnable == NOT_RUNNABLE ==> result == RMI_ERROR_REC) && (RmiRecRunAt(
        old_s,
        run_ptr,
    ).enter.flags.emul_mmio == RMI_EMULATED_MMIO && RecAt(old_s, rec_ptr).emulatable_abort
        != EMULATABLE_ABORT ==> result == RMI_ERROR_REC) && (!Gicv3ConfigIsValid(
        RmiRecRunAt(old_s, run_ptr).enter.gicv3_hcr,
        RmiRecRunAt(old_s, run_ptr).enter.gicv3_lrs,
    ) ==> result == RMI_ERROR_REC) && (RecAt(old_s, rec_ptr).pending != REC_PENDING_NONE ==> result
        == RMI_ERROR_REC) &&
    // Success conditions
    (AddrIsGranuleAligned(run_ptr) && GranuleAccessPermitted(run_ptr, PAS_NS)
        && AddrIsGranuleAligned(rec_ptr) && PaIsDelegable(rec_ptr) && GranuleAt(
        old_s,
        rec_ptr,
    ).state == REC && RealmAt(old_s, RecAt(old_s, rec_ptr).owner).state != REALM_NEW && RealmAt(
        old_s,
        RecAt(old_s, rec_ptr).owner,
    ).state != REALM_SYSTEM_OFF && RecAt(old_s, rec_ptr).state != REC_RUNNING && RecAt(
        old_s,
        rec_ptr,
    ).flags.runnable != NOT_RUNNABLE && !(RmiRecRunAt(old_s, run_ptr).enter.flags.emul_mmio
        == RMI_EMULATED_MMIO && RecAt(old_s, rec_ptr).emulatable_abort != EMULATABLE_ABORT)
        && Gicv3ConfigIsValid(
        RmiRecRunAt(old_s, run_ptr).enter.gicv3_hcr,
        RmiRecRunAt(old_s, run_ptr).enter.gicv3_lrs,
    ) && RecAt(old_s, rec_ptr).pending == REC_PENDING_NONE ==> result == RMI_SUCCESS && RmiRecRunAt(
        new_s,
        run_ptr,
    ).exit.contains_exit_syndrome() && RecAt(new_s, rec_ptr).emulatable_abort == RecAt(
        old_s,
        rec_ptr,
    ).emulatable_abort)
}
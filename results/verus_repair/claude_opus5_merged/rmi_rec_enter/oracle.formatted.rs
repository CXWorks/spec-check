pub open spec fn rmi_rec_enter_spec(
    rec: Address,
    run_ptr: Address,
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, run_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, run_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !GranuleAccessPermitted(old_s, run_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rec,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(old_s, rec).state != REC
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Realm(old_s, Rec(old_s, rec).owner).state
        == REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0 as int))) && (Realm(
        old_s,
        Rec(old_s, rec).owner,
    ).state == REALM_SYSTEM_OFF ==> ResultEqual(result, RMI_ERROR_REALM(1 as int))) && (Rec(
        old_s,
        rec,
    ).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC)) && (Rec(
        old_s,
        rec,
    ).flags.runnable == NOT_RUNNABLE ==> ResultEqual(result, RMI_ERROR_REC)) && ((RecRun(
        old_s,
        run_ptr,
    ).enter.flags.emul_mmio == RMI_EMULATED_MMIO && Rec(old_s, rec).emulatable_abort
        != EMULATABLE_ABORT) ==> ResultEqual(result, RMI_ERROR_REC)) && (!Gicv3ConfigIsValid(
        old_s,
        RecRun(old_s, run_ptr).enter.gicv3_hcr,
        RecRun(old_s, run_ptr).enter.gicv3_lrs,
    ) ==> ResultEqual(result, RMI_ERROR_REC)) && (Rec(old_s, rec).psci_pending
        == PSCI_REQUEST_PENDING ==> ResultEqual(result, RMI_ERROR_REC)) && ((AddrIsGranuleAligned(
        old_s,
        run_ptr,
    ) && PaIsDelegable(old_s, run_ptr) && GranuleAccessPermitted(old_s, run_ptr, PAS_NS)
        && AddrIsGranuleAligned(old_s, rec) && PaIsDelegable(old_s, rec) && !(Granule(
        old_s,
        rec,
    ).state != REC) && !(Realm(old_s, Rec(old_s, rec).owner).state == REALM_NEW) && !(Realm(
        old_s,
        Rec(old_s, rec).owner,
    ).state == REALM_SYSTEM_OFF) && !(Rec(old_s, rec).state == REC_RUNNING) && !(Rec(
        old_s,
        rec,
    ).flags.runnable == NOT_RUNNABLE) && !((RecRun(old_s, run_ptr).enter.flags.emul_mmio
        == RMI_EMULATED_MMIO && Rec(old_s, rec).emulatable_abort != EMULATABLE_ABORT))
        && Gicv3ConfigIsValid(
        old_s,
        RecRun(old_s, run_ptr).enter.gicv3_hcr,
        RecRun(old_s, run_ptr).enter.gicv3_lrs,
    ) && !(Rec(old_s, rec).psci_pending == PSCI_REQUEST_PENDING)) ==> result.is_Ok())
}
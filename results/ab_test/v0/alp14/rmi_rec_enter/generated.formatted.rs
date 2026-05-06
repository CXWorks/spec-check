pub open spec fn RMI_REC_ENTER_spec(s: S, rec_ptr: Address, run_ptr: Address) -> bool {
    let rec = RecAt(s, rec_ptr);
    let run = RmiRecRunAt(s, run_ptr);
    let realm = RealmAt(s, rec.owner);

    (AddrIsGranuleAligned(run_ptr) || ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT)) && (
    GranuleAccessPermitted(s, run_ptr, PAS_NS) || ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT))
        && (AddrIsGranuleAligned(rec_ptr) || ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT)) && (
    PaIsDelegable(s, rec_ptr) || ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT)) && (GranuleAt(
        s,
        rec_ptr,
    ).state == REC || ResultEqual(RMI_ERROR_INPUT, RMI_ERROR_INPUT)) && (realm.state != REALM_NEW
        || ResultEqual(RMI_ERROR_REALM, RMI_ERROR_REALM)) && (realm.state != REALM_SYSTEM_OFF
        || ResultEqual(RMI_ERROR_REALM, RMI_ERROR_REALM)) && (rec.state != REC_RUNNING
        || ResultEqual(RMI_ERROR_REC, RMI_ERROR_REC)) && (rec.flags.runnable != NOT_RUNNABLE
        || ResultEqual(RMI_ERROR_REC, RMI_ERROR_REC)) && ((run.enter.flags.emul_mmio
        != RMI_EMULATED_MMIO || rec.emulatable_abort == EMULATABLE_ABORT) || ResultEqual(
        RMI_ERROR_REC,
        RMI_ERROR_REC,
    )) && (Gicv3ConfigIsValid(s, run.enter.gicv3_hcr, run.enter.gicv3_lrs) || ResultEqual(
        RMI_ERROR_REC,
        RMI_ERROR_REC,
    )) && (rec.pending == REC_PENDING_NONE || ResultEqual(RMI_ERROR_REC, RMI_ERROR_REC))
}
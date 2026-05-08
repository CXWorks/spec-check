pub open spec fn RMI_REC_ENTER_spec(
    old_s: S,
    new_s: S,
    rec_ptr: Address,
    run_ptr: Address,
) -> bool {
    let run = RmiRecRunAt(old_s, run_ptr);
    let rec = RecAt(old_s, rec_ptr);
    let realm = RealmAt(old_s, rec.owner);

    // Failure conditions (ordered by precedence)
    (
    // First group: input validation failures
    (!AddrIsGranuleAligned(run_ptr) ==> ResultEqual(RMI_ERROR_INPUT)) && (!GranuleAccessPermitted(
        old_s,
        run_ptr,
        PAS_NS,
    ) ==> ResultEqual(RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(
        RMI_ERROR_INPUT,
    )) && (!PaIsDelegable(rec_ptr) ==> ResultEqual(RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        rec_ptr,
    ).state != REC ==> ResultEqual(RMI_ERROR_INPUT))
        &&
    // Second group: realm and REC state failures
    (realm.state == REALM_NEW ==> ResultEqual(RMI_ERROR_REALM)) && (realm.state == REALM_SYSTEM_OFF
        ==> ResultEqual(RMI_ERROR_REALM)) && (rec.state == REC_RUNNING ==> ResultEqual(
        RMI_ERROR_REC,
    )) && (rec.flags.runnable == NOT_RUNNABLE ==> ResultEqual(RMI_ERROR_REC)) && ((
    run.enter.flags.emul_mmio == RMI_EMULATED_MMIO && rec.emulatable_abort != EMULATABLE_ABORT)
        ==> ResultEqual(RMI_ERROR_REC)) && (!Gicv3ConfigIsValid(
        old_s,
        run.enter.gicv3_hcr,
        run.enter.gicv3_lrs,
    ) ==> ResultEqual(RMI_ERROR_REC)) && (rec.pending != REC_PENDING_NONE ==> ResultEqual(
        RMI_ERROR_REC,
    ))
        &&
    // Success condition: if no failures, run.exit is updated with exit syndrome
    ((AddrIsGranuleAligned(run_ptr) && GranuleAccessPermitted(old_s, run_ptr, PAS_NS)
        && AddrIsGranuleAligned(rec_ptr) && PaIsDelegable(rec_ptr) && GranuleAt(
        old_s,
        rec_ptr,
    ).state == REC && realm.state != REALM_NEW && realm.state != REALM_SYSTEM_OFF && rec.state
        != REC_RUNNING && rec.flags.runnable != NOT_RUNNABLE && !(run.enter.flags.emul_mmio
        == RMI_EMULATED_MMIO && rec.emulatable_abort != EMULATABLE_ABORT) && Gicv3ConfigIsValid(
        old_s,
        run.enter.gicv3_hcr,
        run.enter.gicv3_lrs,
    ) && rec.pending == REC_PENDING_NONE) ==> (RmiRecRunAt(new_s, run_ptr).exit matches Some(
        exit_info,
    ) && RecAt(new_s, rec_ptr).emulatable_abort == new_s.emul_abt)))
}
pub open spec fn rmi_rec_enter_spec(rec_ptr: Address, run_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let run = RmiRecRunAt(old_s, run_ptr);
    let rec = RecAt(old_s, rec_ptr);
    let realm = RealmAt(old_s, rec.owner);
    let input_fail = !AddrIsGranuleAligned(old_s, run_ptr)
        || !GranuleAccessPermitted(old_s, run_ptr, PAS_NS)
        || !AddrIsGranuleAligned(old_s, rec_ptr)
        || !PaIsDelegable(old_s, rec_ptr)
        || GranuleAt(old_s, rec_ptr).state != REC;
    let realm_new = realm.state == REALM_NEW;
    let realm_off = realm.state == REALM_SYSTEM_OFF;
    let rec_fail = rec.state == REC_RUNNING
        || rec.flags.runnable == NOT_RUNNABLE
        || (run.enter.flags.emul_mmio == RMI_EMULATED_MMIO && rec.emulatable_abort != EMULATABLE_ABORT)
        || !Gicv3ConfigIsValid(old_s, run.enter.gicv3_hcr, run.enter.gicv3_lrs)
        || rec.pending != REC_PENDING_NONE;
    (input_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!input_fail && (realm_new || realm_off || rec_fail)) ==> (
        result.is_Err()
        && ((realm_new && ResultEqual(result, RMI_ERROR_REALM(0)))
            || (realm_off && ResultEqual(result, RMI_ERROR_REALM(1)))
            || (rec_fail && ResultEqual(result, RMI_ERROR_REC)))
    ))
    && ((!input_fail && !realm_new && !realm_off && !rec_fail) ==> result.is_Ok())
}

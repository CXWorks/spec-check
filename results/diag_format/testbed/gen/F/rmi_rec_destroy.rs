pub open spec fn rmi_rec_destroy_spec(rec_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (AddrIsGranuleAligned(old_s, rec_ptr) ==> !ResultEqual(result, RMI_ERROR_INPUT))
    && (PaIsDelegable(old_s, rec_ptr) ==> !ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state == REC ==> !ResultEqual(result, RMI_ERROR_INPUT))
    && (old_s.CurrentRec().state != REC_RUNNING ==> !ResultEqual(result, RMI_ERROR_REC))
    && (GranuleAt(new_s, rec_ptr).state == DELEGATED)
    && (AuxStates(new_s, old_s.CurrentRec().aux, RecAuxCount(old_s, old_s.CurrentRec().owner)) == AuxStates(old_s, old_s.CurrentRec().aux, RecAuxCount(old_s, old_s.CurrentRec().owner)))
    && (new_s.CurrentRealm().num_recs == old_s.CurrentRealm().num_recs - 1)
}
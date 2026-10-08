pub open spec fn rmi_rec_destroy_spec(rec_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (old_s.RecAt(rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (result.is_Ok() ==> (GranuleAt(new_s, rec_ptr).state == DELEGATED && AuxStates(new_s, old_s.RecAt(rec_ptr).aux, old_s.RecAuxCount(old_s, old_s.RecAt(rec_ptr).owner)) && old_s.RealmAt(old_s.RecAt(rec_ptr).owner).num_recs == new_s.RealmAt(old_s.RecAt(rec_ptr).owner).num_recs + 1))
}
pub open spec fn rmi_rec_destroy_spec(result: Result<(), RmiStatusCode>, rec_ptr: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (AddrIsGranuleAligned(rec_ptr)
        && PaIsDelegable(old_s, rec_ptr)
        && GranuleAt(old_s, rec_ptr).state == REC
        && RecAt(old_s, rec_ptr).state != REC_RUNNING
        ==> result.is_Ok()
            && GranuleAt(new_s, rec_ptr).state == DELEGATED
            && AuxStateEqual32(new_s, RecAt(old_s, rec_ptr).aux, RecAuxCount(old_s, RecAt(old_s, rec_ptr).owner), DELEGATED)
            && RealmAt(new_s, RecAt(old_s, rec_ptr).owner).num_recs
                == RealmAt(old_s, RecAt(old_s, rec_ptr).owner).num_recs - 1)
}
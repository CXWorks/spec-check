pub open spec fn rmi_rec_destroy_spec(rec: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rec).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Rec(old_s, rec).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && ((AddrIsGranuleAligned(old_s, rec)
        && PaIsDelegable(old_s, rec)
        && Granule(old_s, rec).state == REC
        && Rec(old_s, rec).state != REC_RUNNING)
        ==> result.is_Ok()
            && Granule(new_s, rec).state == DELEGATED
            && AuxStateEqual(Rec(old_s, rec).aux, RecAuxCount(old_s, Rec(old_s, rec).owner), DELEGATED))
}
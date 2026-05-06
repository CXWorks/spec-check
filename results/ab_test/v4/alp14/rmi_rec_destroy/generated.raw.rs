pub open spec fn RMI_REC_DESTROY_spec(old_s: S, new_s: S, rec_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
    let rd_pre = RecAt(old_s, rec_ptr).owner;
    let realm_pre = RealmAt(old_s, rd_pre);
    let rec_pre = RecAt(old_s, rec_ptr);
    let realm = RealmAt(new_s, rd_pre);
    let rec = RecAt(new_s, rec_ptr);
    
    ((!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (!PaIsDelegable(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT as int)) &&
    (rec_pre.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC as int)) &&
    (result.is_Ok() ==> (
        GranuleAt(new_s, rec_ptr).state == DELEGATED &&
        AuxStateEqual32(old_s, rec_pre.aux, RecAuxCount(old_s, rd_pre), DELEGATED) &&
        realm.num_recs == realm_pre.num_recs - 1
    )))
}
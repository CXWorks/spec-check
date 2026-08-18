pub open spec fn rmi_rec_destroy_spec(rec_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Rec(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (result.is_Ok() ==> Granule(new_s, rec_ptr).state == DELEGATED)
  && (result.is_Ok() ==> AuxStateEqual(new_s, Rec(new_s, rec_ptr).aux, RecAuxCount(new_s, RealmAt(new_s, Rec(new_s, rec_ptr).owner)), DELEGATED))
  && (result.is_Ok() ==> RealmAt(new_s, Rec(new_s, rec_ptr).owner).num_recs == RealmAt(new_s, Rec(new_s, rec_ptr).owner).num_recs - 1)
  && ((AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(Granule(old_s, rec_ptr).state != REC) &&
       !(Rec(old_s, rec_ptr).state == REC_RUNNING))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, rec_ptr).state == Granule(old_s, rec_ptr).state)
  && (result.is_Err()
    ==> RealmAt(new_s, Rec(new_s, rec_ptr).owner).num_recs == RealmAt(old_s, Rec(old_s, rec_ptr).owner).num_recs)
}
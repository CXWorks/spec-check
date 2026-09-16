pub open spec fn rmi_rec_destroy_spec(rec_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTrackedFine(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != GRAN_REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC(0)))
  && (result.is_Ok() ==> GranuleAt(new_s, rec_ptr).state == GRAN_DELEGATED)
  && (result.is_Ok() ==> RealmAt(new_s, RecAt(new_s, rec_ptr).owner).num_recs == RealmAt(new_s, RecAt(new_s, rec_ptr).owner).num_recs - 1)
  && ((AddrIsRmiGranuleAligned(old_s, rec_ptr) &&
       PaIsTrackedFine(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != GRAN_REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rec_ptr).state == GranuleAt(old_s, rec_ptr).state)
  && (result.is_Err()
    ==> RealmAt(new_s, RecAt(new_s, rec_ptr).owner).num_recs == RealmAt(old_s, RecAt(old_s, rec_ptr).owner).num_recs)
}
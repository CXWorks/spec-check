pub open spec fn rmi_rec_destroy_spec(rec_ptr: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((rec_ptr % GRANULE_SIZE) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!can_be_delegated(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() && RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (result.is_Ok() ==> GranuleAt(new_s, rec_ptr).state == DELEGATED)
  && (result.is_Ok() ==> RealmAt(new_s, RecAt(old_s, rec_ptr).owner).num_recs == RealmAt(old_s, RecAt(old_s, rec_ptr).owner).num_recs - 1)
  && ((!( (rec_ptr % GRANULE_SIZE) != 0) &&
       can_be_delegated(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rec_ptr).state == GranuleAt(old_s, rec_ptr).state)
  && (result.is_Err()
    ==> RealmAt(new_s, RecAt(old_s, rec_ptr).owner).num_recs == RealmAt(old_s, RecAt(old_s, rec_ptr).owner).num_recs)
}
pub open spec fn rmi_rec_destroy_spec(rec_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (result.is_Ok() ==> GranuleAt(new_s, rec_ptr).state == DELEGATED)
  && (result.is_Ok() ==> AuxStateEqual16(new_s, RecAt(new_s, rec_ptr).aux, RecAuxCount(new_s, RecAt(new_s, rec_ptr).owner) as int, DELEGATED))
  && (result.is_Ok() ==> RealmAt(new_s, RecAt(new_s, rec_ptr).owner).num_recs == RealmAt(new_s, RecAt(new_s, rec_ptr).owner).num_recs - 1)
  && ((AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(GranuleAt(old_s, rec_ptr).state != REC) &&
       !(RecAt(old_s, rec_ptr).state == REC_RUNNING))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rec_ptr).state == GranuleAt(old_s, rec_ptr).state)
  && (result.is_Err()
    ==> RealmAt(new_s, RecAt(new_s, rec_ptr).owner).num_recs == RealmAt(old_s, RecAt(old_s, rec_ptr).owner).num_recs)
}

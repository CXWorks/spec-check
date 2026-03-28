pub open spec fn rmi_rec_destroy_spec(rec: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rec).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Rec(old_s, rec).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (result.is_Ok() ==> Granule(new_s, rec).state == DELEGATED)
  && (result.is_Ok() ==> AuxStateEqual(Rec(new_s, rec).aux, RecAuxCount(new_s, Rec(new_s, rec).owner), DELEGATED))
  && ((AddrIsGranuleAligned(old_s, rec) &&
       PaIsDelegable(old_s, rec) &&
       !(Granule(old_s, rec).state != REC) &&
       !(Rec(old_s, rec).state == REC_RUNNING))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, rec).state == Granule(old_s, rec).state)
}

pub open spec fn rmi_rec_aux_count_spec(rd: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RmiRecAuxCount(new_s, rd) == new_s.rmi_rec_aux_count)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RmiRecAuxCount(new_s, rd) == RmiRecAuxCount(old_s, rd))
}
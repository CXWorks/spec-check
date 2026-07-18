pub open spec fn rmi_granule_undelegate_spec(addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, addr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> GranuleAt(new_s, addr).gpt != GPT_REALM)
  && (result.is_Ok() ==> GranuleAt(new_s, addr).state == UNDELEGATED)
  && (result.is_Ok() ==> GranuleAt(new_s, addr).content == Zeros())
  && ((AddrIsGranuleAligned(old_s, addr) &&
       PaIsDelegable(old_s, addr) &&
       !(GranuleAt(old_s, addr).state != DELEGATED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, addr).gpt == GranuleAt(old_s, addr).gpt)
  && (result.is_Err()
    ==> GranuleAt(new_s, addr).state == GranuleAt(old_s, addr).state)
  && (result.is_Err()
    ==> GranuleAt(new_s, addr).content == GranuleAt(old_s, addr).content)
}
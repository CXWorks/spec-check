pub open spec fn rmi_granule_delegate_spec(addr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((addr) % GRANULE_SIZE != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) ==> GranuleAt(new_s, addr).state == GPT_REALM)
  && (result.is_Ok() ==> GranuleAt(new_s, addr).state == DELEGATED)
  && (result.is_Ok() ==> GranuleAt(new_s, addr).gpt_entry == GPT_REALM)
  && ((result.is_Ok() || (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT)))
    ==> GranuleAt(new_s, addr).state == GranuleAt(old_s, addr).state)
  && (result.is_Err() && !(ResultEqual(result, RMI_ERROR_INPUT)) && !(ResultEqual(result, RMI_ERROR_INPUT)) && !(ResultEqual(result, RMI_ERROR_INPUT))
    ==> GranuleAt(new_s, addr).state == GranuleAt(old_s, addr).state)
  && (result.is_Err() && !(ResultEqual(result, RMI_ERROR_INPUT)) && !(ResultEqual(result, RMI_ERROR_INPUT)) && !(ResultEqual(result, RMI_ERROR_INPUT))
    ==> GranuleAt(new_s, addr).gpt_entry == GranuleAt(old_s, addr).gpt_entry)
}
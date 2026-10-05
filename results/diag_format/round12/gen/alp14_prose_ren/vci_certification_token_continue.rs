pub open spec fn vci_certification_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: Vcicommandreturncode, len: UInt64, old_s: S, new_s: S) -> bool {
  (addr % KEEPER_EXTENT_SIZE != 0 ==> result == VCI_ERROR_INPUT)
  && (!IsProtectedLba(old_s, Currentvault(old_s), addr) ==> result == VCI_ERROR_INPUT)
  && (LBAMODE(old_s, Currentvault(old_s), addr) == EMPTY ==> result == VCI_ERROR_INPUT)
  && (offset >= KEEPER_EXTENT_SIZE ==> result == VCI_ERROR_INPUT)
  && ((offset + size) < offset ==> result == VCI_ERROR_INPUT)
  && ((offset + size) > KEEPER_EXTENT_SIZE ==> result == VCI_ERROR_INPUT)
  && (Currentworker(old_s).certify_state != CERTIFY_IN_PROGRESS ==> result == VCI_ERROR_STATE)
  && (result == VCI_SUCCESS && Currentworker(new_s).certify_state == NO_CERTIFY_IN_PROGRESS ==> Currentworker(new_s).certify_state == NO_CERTIFY_IN_PROGRESS)
  && ((!(addr % KEEPER_EXTENT_SIZE != 0) &&
       IsProtectedLba(old_s, Currentvault(old_s), addr) &&
       !(LBAMODE(old_s, Currentvault(old_s), addr) == EMPTY) &&
       !(offset >= KEEPER_EXTENT_SIZE) &&
       !((offset + size) < offset) &&
       !((offset + size) > KEEPER_EXTENT_SIZE) &&
       Currentworker(old_s).certify_state == CERTIFY_IN_PROGRESS)
    ==> result == VCI_SUCCESS || result == VCI_INCOMPLETE)
  && (result != VCI_SUCCESS
    ==> Currentworker(new_s).certify_state == Currentworker(old_s).certify_state)
}
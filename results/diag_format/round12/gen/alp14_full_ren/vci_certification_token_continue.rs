pub open spec fn vci_certification_token_continue_spec(addr: Address, offset: UInt64, size: UInt64, result: RsiCommandReturnCode, len: UInt64, old_s: S, new_s: S) -> bool {
  ((addr % EXTENT_SIZE) != 0 ==> result == VCI_ERROR_INPUT)
  && (is_protected_lba(old_s, addr) ==> result == VCI_ERROR_INPUT)
  && (LBAMODE(old_s, addr) == EMPTY ==> result == VCI_ERROR_INPUT)
  && (offset >= KEEPER_EXTENT_SIZE ==> result == VCI_ERROR_INPUT)
  && ((offset + size) < offset ==> result == VCI_ERROR_INPUT)
  && (offset + size > KEEPER_EXTENT_SIZE ==> result == VCI_ERROR_INPUT)
  && (certification_state(old_s, WORKER) != CERTIFY_IN_PROGRESS ==> result == VCI_ERROR_STATE)
  && ((result == VCI_SUCCESS || result == VCI_INCOMPLETE) ==> certification_state(new_s, WORKER) == NO_CERTIFY_IN_PROGRESS)
  && ((!( (addr % EXTENT_SIZE) != 0) &&
       is_protected_lba(old_s, addr) &&
       !(LBAMODE(old_s, addr) == EMPTY) &&
       !(offset >= KEEPER_EXTENT_SIZE) &&
       !((offset + size) < offset) &&
       !(offset + size > KEEPER_EXTENT_SIZE) &&
       certification_state(old_s, WORKER) == CERTIFY_IN_PROGRESS))
    ==> result == VCI_SUCCESS || result == VCI_INCOMPLETE)
  && (result != VCI_SUCCESS && result != VCI_INCOMPLETE
    ==> certification_state(new_s, WORKER) == certification_state(old_s, WORKER))
  && (result == VCI_ERROR_INPUT || result == VCI_ERROR_STATE || result == VCI_ERROR_UNKNOWN
    ==> certification_state(new_s, WORKER) == certification_state(old_s, WORKER))
}
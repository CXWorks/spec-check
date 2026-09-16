pub open spec fn rmi_op_mem_donate_spec(handle: Bits64, list_addr: Address, list_count: UInt64, result: Result<(), RmiStatusCode>, donated_count: UInt64, donate_req: RmiOpMemDonateReq, old_s: S, new_s: S) -> bool {
  (!OperationIncomplete(old_s, handle) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsAligned(old_s, list_addr, 8) ==> result.status == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, list_addr) ==> result.status == RMI_ERROR_INPUT)
  && (result.status == RMI_INCOMPLETE && result.mem == RMI_OP_MEM_DONATE ==> StateOf(old_s, RttAt(new_s, RttAt(new_s, list_addr).base as Address), RttAt(new_s, RttAt(new_s, list_addr).base as Address), 0 as int) == GRAN_INTERNAL)
  && ((result.status == RMI_SUCCESS || result.status == RMI_INCOMPLETE)
    ==> RttAt(new_s, RttAt(new_s, list_addr).base as Address).state == RttEntryState(RttAt(new_s, RttAt(new_s, list_addr).base as Address).state))
  && (RttAt(new_s, RttAt(new_s, list_addr).base as Address).state == RttEntryState(RttAt(new_s, RttAt(new_s, list_addr).base as Address).state))
  && ((!(result.status == RMI_INCOMPLETE && result.mem == RMI_OP_MEM_DONATE))
    ==> RttAt(new_s, RttAt(new_s, list_addr).base as Address).state == RttEntryState(RttAt(new_s, RttAt(new_s, list_addr).base as Address).state))
}
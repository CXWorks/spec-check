pub open spec fn rmi_op_mem_donate_spec(handle: Bits64, list_addr: Address, list_count: UInt64, result: Result<(), RmiStatusCode>, donated_count: UInt64, donate_req: RmiOpMemDonateReq, old_s: S, new_s: S) -> bool {
  (!OperationIncomplete(old_s, handle) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsAligned(old_s, list_addr, 8) ==> result.status == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, list_addr) ==> result.status == RMI_ERROR_INPUT)
  && (result.is_Ok() && result.status == RMI_INCOMPLETE && result.mem == RMI_OP_MEM_DONATE ==> StateOf(new_s, 0, donated_count) == GRAN_INTERNAL)
  && ((result.status == RMI_INCOMPLETE && result.mem == RMI_OP_MEM_DONATE) ==> result.status == RMI_INCOMPLETE && result.data.incomplete.mem == RMI_OP_MEM_REQ_NONE)
  && ((AddrIsAligned(old_s, list_addr, 8) &&
       NonSecureAccessPermitted(old_s, list_addr))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> StateOf(new_s, 0, donated_count) == StateOf(old_s, 0, donated_count))
}
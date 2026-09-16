pub open spec fn rmi_op_mem_reclaim_spec(handle: Bits64, list_addr: Address, list_count: UInt64, result: Result<(RmiStatusCode, RmiResult), (RmiStatusCode, RmiResult)>, reclaim_count: UInt64, old_s: S, new_s: S) -> bool {
  (!OperationIncomplete(old_s, handle) ==> result.0 == RMI_ERROR_INPUT)
  && (!AddrIsAligned(old_s, list_addr, 8) ==> result.0 == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, list_addr) ==> result.0 == RMI_ERROR_INPUT)
  && (result.0 == RMI_INCOMPLETE && result.1.incomplete.mem == RMI_OP_MEM_REQ_NONE)
  && ((result.0 == RMI_SUCCESS) ==> reclaim_count == 0)
  && ((!(OperationIncomplete(old_s, handle)) &&
       AddrIsAligned(old_s, list_addr, 8) &&
       NonSecureAccessPermitted(old_s, list_addr))
    ==> result.0 == RMI_SUCCESS)
}
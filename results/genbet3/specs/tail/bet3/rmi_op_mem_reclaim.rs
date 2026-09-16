pub open spec fn rmi_op_mem_reclaim_spec(handle: Bits64, list_addr: Address, list_count: UInt64, result: Result<(), RmiStatusCode>, reclaim_count: UInt64, old_s: S, new_s: S) -> bool {
  (!OperationIncomplete(old_s, handle) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, list_addr, 8) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!NonSecureAccessPermitted(old_s, list_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() && result.status == RMI_INCOMPLETE && result.data.incomplete.mem == RMI_OP_MEM_REQ_NONE ==> true)
  && ((result.is_Ok() && result.status == RMI_SUCCESS)
    ==> true)
  && ((!(OperationIncomplete(old_s, handle)) &&
       AddrIsAligned(old_s, list_addr, 8) &&
       NonSecureAccessPermitted(old_s, list_addr))
    ==> result.is_Ok())
}
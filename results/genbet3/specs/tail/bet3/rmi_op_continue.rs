pub open spec fn rmi_op_continue_spec(handle: Bits64, flags: RmiContinueFlags, result: Result<(), RmiStatusCode>, donate_req: RmiOpMemDonateReq, old_s: S, new_s: S) -> bool {
  (!OperationIncomplete(old_s, handle) ==> result == RMI_ERROR_INPUT)
  && (result.is_Ok() && OperationIncomplete(old_s, handle) ==> result == RMI_INCOMPLETE)
  && ((OperationIncomplete(old_s, handle) && result.data.incomplete.mem == RMI_OP_MEM_REQ_DONATE) ==> donate_req describes the memory donation requirements of the operation.)
  && ((!(OperationIncomplete(old_s, handle)))
    ==> result.is_Ok())
}
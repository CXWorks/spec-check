pub open spec fn rmi_op_continue_spec(handle: Bits64, flags: RmiContinueFlags, result: RmiResult, donate_req: RmiOpMemDonateReq, old_s: S, new_s: S) -> bool {
  (!OperationIncomplete(old_s, handle) ==> result.status == RMI_ERROR_INPUT)
  && (OperationIncomplete(old_s, handle) && result.data.incomplete.mem == RMI_OP_MEM_REQ_DONATE ==> donate_req describes the memory donation requirements of the operation)
  && ((OperationIncomplete(old_s, handle) && result.data.incomplete.mem == RMI_OP_MEM_REQ_DONATE) ==> donate_req describes the memory donation requirements of the operation)
}
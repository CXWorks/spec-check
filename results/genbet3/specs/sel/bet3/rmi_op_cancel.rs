pub open spec fn rmi_op_cancel_spec(handle: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!OperationIncomplete(old_s, handle) ==> result.status == RMI_ERROR_INPUT)
  && (OperationCanCancel(old_s, handle) != RMM_OP_CAN_CANCEL ==> result.status == RMI_ERROR_INPUT)
  && ((OperationIncomplete(old_s, handle) &&
       !(OperationCanCancel(old_s, handle) != RMM_OP_CAN_CANCEL))
    ==> result.is_Ok())
}
pub open spec fn ffa_msg_send_spec(sender: UInt32, receiver: UInt32, message_size: UInt32, result: FfaCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == FFA_SUCCESS ==> message_size == 0)
  && ((!(result == FFA_SUCCESS))
    ==> message_size == 0)
}
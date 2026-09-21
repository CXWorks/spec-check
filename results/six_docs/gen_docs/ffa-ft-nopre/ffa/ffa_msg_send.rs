pub open spec fn ffa_msg_send_spec(sender: UInt32, receiver: UInt32, message_size: UInt32, flags: UInt32, sender_vcpu_id: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result != RSI_SUCCESS ==> sender == 0)
  && (result != RSI_SUCCESS ==> receiver == 0)
  && (result != RSI_SUCCESS ==> message_size == 0)
  && (result != RSI_SUCCESS ==> flags == 0)
  && (result != RSI_SUCCESS ==> sender_vcpu_id == 0)
  && ((!(sender == 0) &&
       !(receiver == 0) &&
       !(message_size == 0) &&
       !(flags == 0) &&
       !(sender_vcpu_id == 0))
    ==> result == RSI_SUCCESS)
}
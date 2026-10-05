pub open spec fn ffa_msg_send_spec(sender_id: UInt16, receiver_id: UInt16, msg_size: UInt32, result: UInt32, old_s: S, new_s: S) -> bool {
  (result == FFA_SUCCESS ==> RxBufferOf(new_s, receiver_id) contains the message copied from TxBufferOf(new_s, sender_id))
  && (result == FFA_SUCCESS ==> SchedulerInformedOfPendingMessage(new_s, receiver_id))
  && ((!(result == FFA_SUCCESS)) ==> RxBufferOf(new_s, receiver_id) == RxBufferOf(old_s, receiver_id))
  && ((!(result == FFA_SUCCESS)) ==> SchedulerInformedOfPendingMessage(new_s, receiver_id) == SchedulerInformedOfPendingMessage(old_s, receiver_id))
}
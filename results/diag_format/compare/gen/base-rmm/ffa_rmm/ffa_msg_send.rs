pub open spec fn ffa_msg_send_spec(result: UInt32, old_s: S, new_s: S) -> bool {
    (old_s.busy ==> ResultEqual(result, FFA_ERROR_BUSY))
    && (ResultEqual(result, FFA_SUCCESS) ==> (RegistersAreZero(new_s.w1, new_s.w2, new_s.w3, new_s.w4, new_s.w5, new_s.w6, new_s.w7) && RxBufferOf(new_s, old_s.sender_id) == TxBufferOf(new_s, old_s.sender_id) && SchedulerInformedOfPendingMessage(new_s, old_s.receiver_id)))
}
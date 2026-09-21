pub open spec fn ffa_msg_send_spec(sender_receiver_ids: UInt32, reserved: UInt32, message_size: UInt32, flags: UInt32, sender_vcpu_id: UInt32, result: Result<(), FfaStatusCode>, old_s: S, new_s: S) -> bool {
  true
}
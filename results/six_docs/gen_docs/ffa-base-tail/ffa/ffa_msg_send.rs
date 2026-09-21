pub open spec fn ffa_msg_send_spec(result: int, old_s: S, new_s: S) -> bool {
    // Failure: INVALID_PARAMETERS if Sender ID or Receiver ID is invalid
    // Failure: INVALID_PARAMETERS if reserved bits in Flags are non-zero
    // Failure: INVALID_PARAMETERS if reserved/unused parameter registers (w2, w6-w7, x6-x7) are non-zero
    // Failure: NO_MEMORY if Receiver RX buffer size is insufficient
    // Failure: BUSY if Receiver RX buffer is not available (and optionally save Sender ID for notification)
    // Failure: DENIED if Callee is not in a state to handle this request
    // Failure: NOT_SUPPORTED if function is not implemented at this FF-A instance
    // Failure: ABORTED, INTERRUPTED, NO_DATA, NO_MEMORY, RETRY, NOT_READY as per general FFA error handling
    // Success: result == FFA_SUCCESS
    // Note: The spec text does not define constraints on the state transition (old_s -> new_s) for this command.
    // It describes responsibilities (copying, locking, unlocking, notifying) but does not specify
    // how these responsibilities map to the abstract state S. Therefore, we cannot assert any
    // state invariants (e.g., "message copied", "TX buffer unlocked") without inventing state fields.
    // We only assert the result code constraints.
    (result == FFA_SUCCESS)
}
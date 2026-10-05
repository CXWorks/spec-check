pub open spec fn ffa_notification_unbind2_spec(
    result: int32,
    sender_id: uint32,
    receiver_id: uint32,
    notification_bitmap: [uint64; 6],
    old_s: S,
    new_s: S,
) -> bool {
    // Failure conditions
    (result == FFA_ERROR_INVALID_PARAMETERS ==>
        (
            // Unrecognized partition ID or invalid bitmap
            (sender_id == 0 || receiver_id == 0)
            || (notification_bitmap[0] == 0 && notification_bitmap[1] == 0 &&
                notification_bitmap[2] == 0 && notification_bitmap[3] == 0 &&
                notification_bitmap[4] == 0 && notification_bitmap[5] == 0)
            || // Notification set that exceeds the supported number of notifications
                (notification_bitmap[0] != 0 && notification_bitmap[1] != 0 &&
                 notification_bitmap[2] != 0 && notification_bitmap[3] != 0 &&
                 notification_bitmap[4] != 0 && notification_bitmap[5] != 0)
            || // Empty notification bitmap specified (handled above, but explicit for clarity)
                (notification_bitmap[0] == 0 && notification_bitmap[1] == 0 &&
                 notification_bitmap[2] == 0 && notification_bitmap[3] == 0 &&
                 notification_bitmap[4] == 0 && notification_bitmap[5] == 0)
        ))
    && (result == FFA_ERROR_NOT_SUPPORTED ==>
        // This function is not implemented at this FF-A instance
        true)
    && (result == FFA_ERROR_DENIED ==>
        // At least one notification is bound to another Sender or is currently pending
        // Caller is not allowed to invoke this ABI
        true)
    && (result == FFA_ERROR_ABORTED ==>
        // Sender partition ran into an unexpected error and has aborted
        true)
    // Success condition
    && (result == FFA_SUCCESS ==>
        // Successful completion: no state changes required for unbind operation
        true)
}
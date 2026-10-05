pub open spec fn ffa_notification_unbind_spec(result: int32, old_s: S, new_s: S) -> bool {
    // Failure: Unrecognized partition ID or invalid bitmap
    (result == FFA_ERROR_INVALID_PARAMETERS ==> (
        !IsPartitionIdValid(old_s, ExtractSenderId(old_s)) ||
        !IsPartitionIdValid(old_s, ExtractReceiverId(old_s)) ||
        !IsNotificationBitmapValid(old_s)
    ))
    // Failure: Function not implemented at this FF-A instance
    (result == FFA_ERROR_NOT_SUPPORTED ==> true)
    // Failure: At least one notification is bound to another Sender or is currently pending
    (result == FFA_ERROR_DENIED ==> (
        ExistsBoundNotification(old_s, ExtractSenderId(old_s), ExtractNotificationBitmap(old_s)) ||
        ExistsPendingNotification(old_s, ExtractSenderId(old_s), ExtractNotificationBitmap(old_s))
    ))
    // Failure: Caller is not allowed to invoke this ABI
    (result == FFA_ERROR_DENIED ==> !IsCallerAllowed(old_s, ExtractSenderId(old_s)))
    // Failure: Sender partition ran into an unexpected error and has aborted
    (result == FFA_ERROR_ABORTED ==> IsPartitionAborted(old_s, ExtractSenderId(old_s)))
    // Success: Returns FFA_SUCCESS
    (result == FFA_SUCCESS ==> true)
}

// Helper predicates (uninterpreted as per spec constraints)
pub open spec fn IsPartitionIdValid(s: S, id: uint32) -> bool { ... }
pub open spec fn IsNotificationBitmapValid(s: S) -> bool { ... }
pub open spec fn ExistsBoundNotification(s: S, sender_id: uint32, bitmap: uint64) -> bool { ... }
pub open spec fn ExistsPendingNotification(s: S, sender_id: uint32, bitmap: uint64) -> bool { ... }
pub open spec fn IsCallerAllowed(s: S, sender_id: uint32) -> bool { ... }
pub open spec fn IsPartitionAborted(s: S, sender_id: uint32) -> bool { ... }
pub open spec fn ExtractSenderId(s: S) -> uint32 { ... }
pub open spec fn ExtractReceiverId(s: S) -> uint32 { ... }
pub open spec fn ExtractNotificationBitmap(s: S) -> uint64 { ... }
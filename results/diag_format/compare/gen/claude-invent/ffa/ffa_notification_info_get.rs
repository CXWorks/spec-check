pub open spec fn ffa_notification_info_get_spec(function_id: UInt32, result: Result<(), FfaErrorCode>, flags: UInt64, id_lists: Seq<UInt64>, old_s: S, new_s: S) -> bool {
    (!FfaNotificationInfoGetImplemented(old_s) ==> (ResultEqual(result, NOT_SUPPORTED) && new_s == old_s))
    && ((FfaNotificationInfoGetImplemented(old_s) && !HasPendingNotificationInfo(old_s)) ==> (ResultEqual(result, NO_DATA) && new_s == old_s))
    && ((FfaNotificationInfoGetImplemented(old_s) && HasPendingNotificationInfo(old_s)) ==> (
        result.is_Ok()
        && (flags & 0x7Eu64) == 0u64
        && ((flags >> 7u64) & 0x1Fu64) >= 1u64
        && (function_id == 0x84000083u32 ==> (
            (flags >> 32u64) == 0u64
            && (flags & 0x800u64) == 0u64
            && ((flags >> 7u64) & 0x1Fu64) <= 10u64
        ))
        && (function_id == 0xC4000083u32 ==> (
            (flags >> 52u64) == 0u64
            && ((flags >> 7u64) & 0x1Fu64) <= 20u64
        ))
        && IdListsEncodePendingNotificationInfo(old_s, function_id, flags, id_lists)
        && (((flags & 0x1u64) == 1u64) <==> HasPendingNotificationInfo(new_s))
        && PendingNotificationInfoRetrievedOnce(old_s, new_s, flags, id_lists)
    ))
}

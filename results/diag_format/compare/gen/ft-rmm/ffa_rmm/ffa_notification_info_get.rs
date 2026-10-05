pub open spec fn ffa_notification_info_get_spec(result: Result<FFAStatus, ()>, pending_flags: UInt64, id_lists: [UInt32; 5], old_s: S, new_s: S) -> bool {
  (!PendingNotificationInfoAvailable(old_s) ==> ResultEqual(result, NO_DATA))
  && (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_INFO_GET) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result == FFA_SUCCESS ==> ReturnedFunction(new_s) == FFA_SUCCESS)
  && (result == FFA_SUCCESS ==> pending_flags[0] == (AllIdListsRetrieved(new_s) ? 0 : 1))
  && (result == FFA_SUCCESS ==> pending_flags[6..1] == 0)
  && (result == FFA_SUCCESS ==> pending_flags[11..7] == NumIdListsReturned(new_s))
  && (result == FFA_SUCCESS && IsSmc32(old_s) ==> pending_flags[11] == 0)
  && (result == FFA_SUCCESS ==> for 1 <= i <= NumIdListsReturned(new_s): IdListSize(new_s, i) == pending_flags[(2 * i) - 1 + 12 .. (2 * (i - 1)) + 12] + 1)
  && (result == FFA_SUCCESS && IsSmc64(old_s) ==> pending_flags[63..52] == 0)
  && (result == FFA_SUCCESS ==> for each list L: 1 <= IdListSize(new_s, L) <= 4)
  && (result == FFA_SUCCESS ==> for each endpoint E with only pending global notifications: E is returned in a list of size 1)
  && (result == FFA_SUCCESS ==> for each endpoint E with pending per-vCPU notifications: each list for E has E as its first element, followed by the IDs of vCPUs with pending notifications, and has size > 1)
  && (result == FFA_SUCCESS ==> the first ID of the first list is in id_lists[15:0] (SMC32) or id_lists[15:0] (SMC64); each later list starts at the bit position that follows the IDs of the previous list, in the same or a higher numbered register)
  && (result == FFA_SUCCESS ==> TotalIdsReturned(new_s) <= (IsSmc32(old_s) ? 10 : 20))
  && (result == FFA_SUCCESS && IsNonSecureVirtualInstance(old_s) ==> every returned endpoint has its scheduler implemented in the calling VM)
  && (result == FFA_SUCCESS ==> no returned ID list is returned again in a later invocation)
  && ((PendingNotificationInfoAvailable(old_s) &&
       IsImplementedAtInstance(old_s, FFA_NOTIFICATION_INFO_GET))
    ==> result == FFA_SUCCESS)
  && (result != FFA_SUCCESS
    ==> ReturnedFunction(new_s) == FFA_ERROR)
  && (result != FFA_SUCCESS
    ==> pending_flags[0] == 0)
  && (result != FFA_SUCCESS
    ==> pending_flags[6..1] == 0)
  && (result != FFA_SUCCESS
    ==> pending_flags[11..7] == 0)
  && (result != FFA_SUCCESS
    ==> IsSmc32(old_s) ==> pending_flags[11] == 0)
  && (result != FFA_SUCCESS
    ==> IsSmc64(old_s) ==> pending_flags[63..52] == 0)
  && (result != FFA_SUCCESS
    ==> TotalIdsReturned(new_s) == 0)
  && (!(PendingNotificationInfoAvailable(old_s) &&
       IsImplementedAtInstance(old_s, FFA_NOTIFICATION_INFO_GET))
    ==> result != FFA_SUCCESS)
}
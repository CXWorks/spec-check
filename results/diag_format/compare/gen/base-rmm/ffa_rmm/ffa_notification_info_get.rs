pub open spec fn ffa_notification_info_get_spec(result: Int32, pending_flags: UInt64, id_lists: [UInt64; 5], old_s: S, new_s: S) -> bool {
    (!PendingNotificationInfoAvailable(old_s) ==> ResultEqual(result, NO_DATA))
    && (!IsImplementedAtInstance(old_s, FFA_NOTIFICATION_INFO_GET) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ReturnedFunction(old_s) == FFA_SUCCESS ==> (
        pending_flags[0] == (AllIdListsRetrieved(old_s) as UInt32)
        && pending_flags[6:1] == 0
        && pending_flags[11:7] == NumIdListsReturned(old_s)
        && (IsSmc32(old_s) ==> pending_flags[11] == 0)
        && (IsSmc64(old_s) ==> pending_flags[63:52] == 0)
        && (forall i: Int where 1 <= i <= NumIdListsReturned(old_s): IdListSize(old_s, i) == pending_flags[(2 * i) - 1 + 12 : (2 * (i - 1)) + 12] + 1)
        && (forall L: Endpoint: 1 <= IdListSize(old_s, L) <= 4)
        && (forall E: Endpoint where PendingGlobalNotifications(old_s, E): E is returned in a list of size 1)
        && (forall E: Endpoint where PendingPerVcpuNotifications(old_s, E): exists L: Endpoint where FirstElement(L) == E && Size(L) > 1)
        && (first_id_of_first_list(old_s) == (IsSmc32(old_s) ? pending_flags[15:0] : pending_flags[15:0]))
        && (forall L: Endpoint: TotalIdsReturned(old_s) <= (IsSmc32(old_s) ? 10 : 20))
        && (IsNonSecureVirtualInstance(old_s) ==> forall E: Endpoint where ReturnedEndpoint(old_s, E): SchedulerImplementedInCallingVm(old_s, E))
        && (forall L: Endpoint: !ReturnedAgainInLaterInvocation(old_s, L))
    ))
}
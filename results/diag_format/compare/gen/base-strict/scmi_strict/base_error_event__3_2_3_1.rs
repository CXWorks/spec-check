pub open spec fn base_error_event__3_2_3_1_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!old_s.agent_id_is_valid() ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!old_s.protocol_id_is_valid() ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!old_s.command_list_is_valid() ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (result.is_Ok() ==> (
        (old_s.agent_id_is_valid() ==> IsRegisteredForBaseErrorNotifications(old_s.agent_id))
        && (old_s.is_initial_boot() ==> !BaseErrorNotificationsEnabled(old_s.agent_id))
        && (Bits64(old_s.error_status, 31, 31) == 1 ==> !PlatformCanProcessCommands(old_s))
        && (Bits64(old_s.error_status, 31, 31) == 0 ==> (PlatformIsOperational(old_s) && SomeCommandsFailed(old_s)))
        && (Bits64(old_s.error_status, 30, 10) == 0)
        && (Bits64(old_s.error_status, 9, 0) == old_s.command_count)
        && (forall|i: UInt32| i < old_s.command_count ==> IsFailedCommandEntry(old_s.command_list, i))
    ))
}
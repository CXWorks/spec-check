pub open spec fn base_error_event__3_2_3_1_spec(agent_id: UInt32, error_status: UInt32, command_list: [UInt32; 1], result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (NotificationSentTo(new_s, agent_id) ==> (IsRegisteredForBaseErrorNotifications(new_s, agent_id) && PlatformImplementsBaseErrorNotifications(new_s)))
  && (IsInitialBoot(new_s, agent_id) ==> !BaseErrorNotificationsEnabled(new_s, agent_id))
  && (Bits(error_status, 31, 31) == 1 ==> !PlatformCanProcessCommands(new_s))
  && (Bits(error_status, 31, 31) == 0 ==> (PlatformIsOperational(new_s) && SomeCommandsFailed(new_s)))
  && (Bits(error_status, 30, 10) == 0)
  && (Bits(error_status, 9, 0) == 0)
  && (forall (i: UInt32), i < Bits(error_status, 9, 0) ==> IsFailedCommandEntry(new_s, command_list, i))
  && ((!(NotificationSentTo(old_s, agent_id)) &&
       !(IsInitialBoot(old_s, agent_id)) &&
       !(Bits(error_status, 31, 31) == 1) &&
       !(Bits(error_status, 31, 31) == 0) &&
       !(Bits(error_status, 30, 10) == 0) &&
       !(Bits(error_status, 9, 0) == 0) &&
       !(forall (i: UInt32), i < Bits(error_status, 9, 0) ==> IsFailedCommandEntry(old_s, command_list, i)))
    ==> result == RSI_SUCCESS)
}
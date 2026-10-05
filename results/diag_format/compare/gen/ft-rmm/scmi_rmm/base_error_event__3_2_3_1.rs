pub open spec fn base_error_event__3_2_3_1_spec(agent: UInt32, error_status: UInt32, command_list: [UInt32; 1], result: BaseErrorEventResult, old_s: S, new_s: S) -> bool {
  (IsRegisteredForBaseErrorNotification(old_s, agent) && PlatformImplementsBaseErrorNotification(old_s) ==> NotificationSentTo(new_s, agent))
  && (IsInitialBoot(old_s, agent) ==> !IsRegisteredForBaseErrorNotification(new_s, agent))
  && (IsFatalError(old_s) ==> (error_status & 0x80000000) != 0)
  && (!(IsFatalError(old_s)) ==> (error_status & 0x80000000) == 0)
  && ((error_status >> 10) & 0x3FF == 0)
  && ((error_status & 0x3FF) == command_list.len())
  && (!(IsFatalError(old_s)) ==> command_list == CommandsNotProcessed())
  && ((!IsRegisteredForBaseErrorNotification(old_s, agent) || !PlatformImplementsBaseErrorNotification(old_s))
    ==> !NotificationSentTo(new_s, agent))
  && (!IsInitialBoot(old_s, agent)
    ==> IsRegisteredForBaseErrorNotification(new_s, agent))
  && (!IsFatalError(old_s) && (error_status & 0x80000000) == 0)
  ==> (error_status & 0x80000000) == 0
  && (result == BaseErrorEventResult::Success
    ==> (error_status & 0x80000000) == (IsFatalError(old_s) as UInt32))
}
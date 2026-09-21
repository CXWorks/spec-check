pub open spec fn 3.2.2.10_base_notify_errors_spec(notify_enable: UInt32, result: Result<int32, ()>, old_s: S, new_s: S) -> bool {
  (result == SUCCESS ==> notify_enable == 0)
  && (result == SUCCESS ==> notify_enable == 1)
  && ((!(result == SUCCESS))
    ==> notify_enable == notify_enable)
}
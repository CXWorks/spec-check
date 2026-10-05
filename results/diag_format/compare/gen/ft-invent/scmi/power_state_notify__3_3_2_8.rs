pub open spec fn power_state_notify__3_3_2_8_spec(domain_id: UInt32, notify_enable: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  ((notify_enable & 1) == 0 ==> result == RSI_SUCCESS)
  && (result == RSI_NOT_FOUND ==> (RealmAt(new_s, domain_id as int).power_state_notify == RealmAt(old_s, domain_id as int).power_state_notify))
  && (result == RSI_INVALID_PARAMETERS ==> (RealmAt(new_s, domain_id as int).power_state_notify == RealmAt(old_s, domain_id as int).power_state_notify))
  && ((!( (notify_enable & 1) == 0 ))
    ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS
    ==> RealmAt(new_s, domain_id as int).power_state_notify != RealmAt(old_s, domain_id as int).power_state_notify)
}
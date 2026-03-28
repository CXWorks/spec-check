pub open spec fn rsi_rdev_get_state_spec(vdev_id: Bits64, inst_id: UInt64, result: RsiCommandReturnCode, state: RsiDeviceState, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> Equal(state, RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state))
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int))
    ==> result == RSI_SUCCESS)
}

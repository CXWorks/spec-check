pub open spec fn rsi_rdev_get_interface_report_spec(vdev_id: Bits64, inst_id: UInt64, version_max: UInt64, result: RsiCommandReturnCode, version: UInt64, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!VdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && ((VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state != RDEV_LOCKED && VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state != RDEV_STARTED) ==> result == RSI_ERROR_DEVICE)
  && (result == RSI_SUCCESS && VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state == RDEV_LOCKED ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == RDEV_LOCKED_BUSY)
  && (result == RSI_SUCCESS && VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state == RDEV_STARTED ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == RDEV_STARTED_BUSY)
  && (result == RSI_SUCCESS ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RDEV_OP_GET_INTERFACE_REPORT)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       VdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       !((VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state != RDEV_LOCKED && VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state != RDEV_STARTED)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state)
  && (result != RSI_SUCCESS
    ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state)
  && (result != RSI_SUCCESS
    ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).operation)
  && (!(result == RSI_SUCCESS && (VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state == RDEV_LOCKED)) ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state)
}

pub open spec fn rsi_rdev_get_interface_report_spec(vdev_id: Bits64, inst_id: UInt64, version_max: UInt64, result: RsiCommandReturnCode, version: UInt64, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && ((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_LOCKED ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_LOCKED_BUSY)
  && (result == RSI_SUCCESS && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_STARTED ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RDEV_STARTED_BUSY)
  && (result == RSI_SUCCESS ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RDEV_OP_GET_INTERFACE_REPORT)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       !((RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_LOCKED && RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state != RDEV_STARTED)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
  && (result != RSI_SUCCESS
    ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).operation)
  && (!(result == RSI_SUCCESS && (RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state == RDEV_LOCKED)) ==> RdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).state == RdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).state)
}

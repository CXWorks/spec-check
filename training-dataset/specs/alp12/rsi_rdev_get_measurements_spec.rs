pub open spec fn rsi_rdev_get_measurements_spec(vdev_id: Bits64, inst_id: UInt64, params_addr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!VdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) ==> result == RSI_ERROR_INPUT)
  && ((RsiDevMeasureParamsAt(old_s, params_addr).indices[0] == '1' || RsiDevMeasureParamsAt(old_s, params_addr).indices[255] == '1') ==> result == RSI_ERROR_INPUT)
  && ((VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state != RDEV_UNLOCKED && VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state != RDEV_LOCKED && VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state != RDEV_STARTED) ==> result == RSI_ERROR_DEVICE)
  && (result == RSI_SUCCESS && VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state == RDEV_UNLOCKED ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == RDEV_UNLOCKED_BUSY)
  && (result == RSI_SUCCESS && VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state == RDEV_LOCKED ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == RDEV_LOCKED_BUSY)
  && (result == RSI_SUCCESS && VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state == RDEV_STARTED ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == RDEV_STARTED_BUSY)
  && (result == RSI_SUCCESS ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == RDEV_OP_GET_MEASUREMENTS)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       VdevIdsAreValid(old_s, CurrentRealm(old_s), vdev_id, inst_id as int) &&
       !((RsiDevMeasureParamsAt(old_s, params_addr).indices[0] == '1' || RsiDevMeasureParamsAt(old_s, params_addr).indices[255] == '1')) &&
       !((VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state != RDEV_UNLOCKED && VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state != RDEV_LOCKED && VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state != RDEV_STARTED)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state)
  && (result != RSI_SUCCESS
    ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state)
  && (result != RSI_SUCCESS
    ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state)
  && (result != RSI_SUCCESS
    ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).operation == VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).operation)
  && (!(result == RSI_SUCCESS && (VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state == RDEV_UNLOCKED)) ==> VdevFromInstId(new_s, CurrentRealm(new_s),inst_id as int).rdev_state == VdevFromInstId(old_s, CurrentRealm(old_s),inst_id as int).rdev_state)
}

pub open spec fn rsi_rdev_get_instance_id_spec(vdev_id: Bits64, result: RsiCommandReturnCode, inst_id: UInt64, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!VdevIdIsValid(old_s, CurrentRealm(old_s), vdev_id) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> inst_id == VdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).inst_id)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       VdevIdIsValid(old_s, CurrentRealm(old_s), vdev_id))
    ==> result == RSI_SUCCESS)
}

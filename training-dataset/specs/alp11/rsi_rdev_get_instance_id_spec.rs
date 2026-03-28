pub open spec fn rsi_rdev_get_instance_id_spec(vdev_id: Bits64, result: RsiCommandReturnCode, inst_id: UInt64, old_s: S, new_s: S) -> bool {
  (CurrentRealm(old_s).feat_da != FEATURE_TRUE ==> result == RSI_ERROR_STATE)
  && (!RdevIdIsValid(old_s, CurrentRealm(old_s), vdev_id) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> inst_id == VdevAt(new_s, RdevFromVdevId(new_s, CurrentRealm(new_s),vdev_id).vdev_ptr).inst_id)
  && ((!(CurrentRealm(old_s).feat_da != FEATURE_TRUE) &&
       RdevIdIsValid(old_s, CurrentRealm(old_s), vdev_id))
    ==> result == RSI_SUCCESS)
}

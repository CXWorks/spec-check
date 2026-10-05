pub open spec fn telemetry_config_set__3_12_4_10_spec(group_identifier: UInt32, control: UInt32, sampling_rate: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> result == RMI_ERROR_INPUT)
  && (result.is_Err() ==> result == RMI_ERROR_REALM)
  && (result.is_Err() ==> result == RMI_ERROR_REC)
  && (result.is_Err() ==> result == RMI_ERROR_RTT)
  && (result.is_Ok() ==> true)
}
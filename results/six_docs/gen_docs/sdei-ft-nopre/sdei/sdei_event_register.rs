pub open spec fn sdei_event_register_spec(event: int32, entry_point_address: UInt64, ep_argument: UInt64, flags: UInt64, affinity: UInt64, result: Result<(), SdeiStatusCode>, old_s: S, new_s: S) -> bool {
  true
}
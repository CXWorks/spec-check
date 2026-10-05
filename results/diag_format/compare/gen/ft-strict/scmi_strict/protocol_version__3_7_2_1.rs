pub open spec fn protocol_version__3_7_2_1_spec(status: Result<(), RmiStatusCode>, version: UInt32, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() ==> version == 0x30001)
  && ((!(result.is_Ok()))
    ==> version == 0)
}
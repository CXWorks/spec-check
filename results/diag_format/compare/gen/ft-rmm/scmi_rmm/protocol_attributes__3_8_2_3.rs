pub open spec fn protocol_attributes__3_8_2_3_spec(status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  StatusIsSuccess(status)
  && (attributes[31:16] == 0)
  && (attributes[15:0] == NumResetDomains())
  && ((!(StatusIsSuccess(status)))
    ==> (attributes[31:16] == 0))
  && ((!(StatusIsSuccess(status)))
    ==> (attributes[15:0] == NumResetDomains()))
}
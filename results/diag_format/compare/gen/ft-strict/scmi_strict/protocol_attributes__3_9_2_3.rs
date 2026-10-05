pub open spec fn protocol_attributes__3_9_2_3_spec(status: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
  ResultEqual(status, SUCCESS)
  && (Bits(attributes, 31, 16) == 0)
  && (Bits(attributes, 15, 0) == NumVoltageDomains())
  && ((!(ResultEqual(status, SUCCESS)))
    ==> (Bits(attributes, 31, 16) == 0))
  && ((!(ResultEqual(status, SUCCESS)))
    ==> (Bits(attributes, 15, 0) == NumVoltageDomains()))
}
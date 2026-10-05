pub open spec fn telemetry_de_description__3_12_4_6_spec(desc_index: UInt32, result: RsiCommandReturnCode, num_remaining: UInt16, num_returned: UInt16, desc: [DE_DESC; 1], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> num_returned == 1)
  && (result == RSI_SUCCESS ==> desc[0] == DeDescriptorAt(old_s, desc_index))
  && (result == RSI_SUCCESS ==> num_remaining == NumRemainingDeDescriptors(old_s, desc_index, num_returned))
  && (result == RSI_SUCCESS ==> forall i < 1: IsDeDescFormat(desc[i]))
  && ((!(result == RSI_SUCCESS))
    ==> num_remaining == NumRemainingDeDescriptors(old_s, desc_index, 0))
  && ((!(result == RSI_SUCCESS))
    ==> num_returned == 0)
}
pub open spec fn rsi_version_spec(req: UInt64, lower: UInt64, higher: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> lower == req)
  && (result == RSI_SUCCESS ==> higher == RsiVersionHighest(old_s))
  && ((!(result == RSI_SUCCESS))
    ==> lower <= higher)
  && (result == RSI_ERROR_INPUT
    ==> lower <= req)
  && (result == RSI_ERROR_INPUT
    ==> higher == RsiVersionHighest(old_s))
  && ((result != RSI_SUCCESS && result != RSI_ERROR_INPUT)
    ==> lower == higher)
  && (result != RSI_SUCCESS
    ==> higher == RsiVersionHighest(old_s))
  && (S == S ==> RealmAt(new_s, 0).rtt_level_start == RealmAt(old_s, 0).rtt_level_start)
}
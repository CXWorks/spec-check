pub open spec fn rsi_version_spec(req: RsiInterfaceVersion, result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INPUT && (RmmRsi(s).supported_versions.len() > 0) ==> lower < req)
  && (result == RSI_ERROR_INPUT && (RmmRsi(s).supported_versions.len() > 0) ==> higher > lower)
  && (result == RSI_ERROR_INPUT && RmmRsi(s).supported_versions.len() == 0 ==> lower == higher)
  && (result == RSI_SUCCESS ==> lower == req)
  && (result == RSI_SUCCESS ==> higher == RmmRsi(new_s).supported_versions[RmmRsi(new_s).supported_versions.len() - 1])
  && ((!(result == RSI_ERROR_INPUT && (RmmRsi(old_s).supported_versions.len() > 0)) &&
       result == RSI_SUCCESS)
    ==> lower == req)
  && (result != RSI_SUCCESS
    ==> lower == RmmRsi(new_s).supported_versions[RmmRsi(new_s).supported_versions.len() - 1])
  && (result != RSI_SUCCESS
    ==> higher == RmmRsi(new_s).supported_versions[RmmRsi(new_s).supported_versions.len() - 1])
  && ((!(result == RSI_ERROR_INPUT && (RmmRsi(old_s).supported_versions.len() > 0)) &&
       result == RSI_SUCCESS)
    ==> higher == RmmRsi(new_s).supported_versions[RmmRsi(new_s).supported_versions.len() - 1])
}
pub open spec fn pinctrl_name_get__3_11_2_11_spec(identifier: UInt16, selector: UInt2, result: Result<Int32, [UInt8; 64], UInt32>, old_s: S, new_s: S) -> bool {
  (!PinctrlEntityExists(old_s, selector, identifier) ==> ResultEqual(result.0, NOT_FOUND))
  && (result.0 == SUCCESS ==> result.0 == SUCCESS)
  && (result.0 == SUCCESS ==> result.2 == PinctrlExtendedName(old_s, selector, identifier))
  && (result.0 == SUCCESS ==> IsNullTerminatedAscii(result.2, 64))
  && (result.0 == SUCCESS ==> result.1 == 0)
  && ((PinctrlEntityExists(old_s, selector, identifier))
    ==> ResultEqual(result.0, SUCCESS))
  && (result.0 != SUCCESS
    ==> result.2 == PinctrlExtendedName(old_s, selector, identifier))
  && (result.0 != SUCCESS
    ==> IsNullTerminatedAscii(result.2, 64))
  && (result.0 != SUCCESS
    ==> result.1 == 0)
  && (result.is_Ok()
    ==> result.1 == 0)
}
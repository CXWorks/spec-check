pub open spec fn mem_protect_spec(fid: UInt, enable: UInt, result: Int, old_s: S, new_s: S) -> bool {
  (!IsMemProtectImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (ResultEqual(result, NOT_SUPPORTED) ==> MemProtectEnabled(new_s) == false)
  && (result == 0 || result == 1 ==> ResultEqual(result, Old(new_s, MemProtectEnabled()) ? 1 : 0))
  && ((enable != 0) == MemProtectEnabled(new_s))
  && ((!(IsMemProtectImplemented(old_s))) ==> MemProtectEnabled(new_s) == false)
}
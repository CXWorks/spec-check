pub open spec fn sbi_get_spec_version_spec(ret: struct sbiret, old_s: S, new_s: S) -> bool {
  (ret[23:0] == CurrentSbiSpecMinorVersion())
  && (ret[30:24] == CurrentSbiSpecMajorVersion())
  && (ret[31] == 0)
  && ((XLEN > 32) ==> (ret[XLEN-1:32] == 0))
  && ((!(ret[23:0] == CurrentSbiSpecMinorVersion())) &&
       !(ret[30:24] == CurrentSbiSpecMajorVersion()))
  && (!(ret[31] == 0))
  && !((XLEN > 32) && !(ret[XLEN-1:32] == 0))
  && ((ret[23:0] != CurrentSbiSpecMinorVersion()) ||
       ret[30:24] != CurrentSbiSpecMajorVersion())
  && (ret[31] != 0)
  && ((!(XLEN > 32)) ||
       ret[XLEN-1:32] != 0)
}
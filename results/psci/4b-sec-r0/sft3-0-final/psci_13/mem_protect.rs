pub open spec fn mem_protect_spec(enable: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> mem_protect(new_s, enable) == mem_protect(old_s, enable))
  && (!result.is_Err() ==> mem_protect(new_s, enable) != mem_protect(old_s, enable))
}
pub open spec fn mem_protect_check_range_spec(base: Address, length: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> AddrIsNonSecure(new_s, base + length - 1))
  && (result.is_Err()
    ==> AddrIsNonSecure(new_s, base + length - 1))
}
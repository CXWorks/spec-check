pub open spec fn sbi_mpxy_get_shmem_size_spec(error: SbiReturnCode, uvalue: UInt64, old_s: S, new_s: S) -> bool {
  (ResultEqual(error, SBI_SUCCESS))
  && (forall (h: Hart), MpxyShmemSize(h) == uvalue)
  && (uvalue >= 4096)
  && (uvalue % 4096 == 0)
  && (forall (c: MpxyChannel), uvalue >= MsgDataMaxLen(c))
  && ((!(ResultEqual(error, SBI_SUCCESS)))
    ==> (forall (h: Hart), MpxyShmemSize(h) == MpxyShmemSize(old_s, h)))
}
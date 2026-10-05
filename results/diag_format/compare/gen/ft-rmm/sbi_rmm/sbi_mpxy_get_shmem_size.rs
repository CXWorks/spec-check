pub open spec fn sbi_mpxy_get_shmem_size_spec(eid: UInt, fid: UInt, error: SbiErrorCode, uvalue: UInt, old_s: S, new_s: S) -> bool {
  (ResultEqual(error, SBI_SUCCESS))
  && (uvalue == MpxyShmemSize())
  && (forall (hart: int), MpxyShmemSizeOnHart(hart) == uvalue)
  && (uvalue >= 4096)
  && ((uvalue % 4096) == 0)
  && (forall (chan in MpxyChannels()), uvalue >= MsgDataMaxLen(chan))
  && ((!(ResultEqual(error, SBI_SUCCESS)))
    ==> (MpxyShmemSize() == MpxyShmemSize(old_s)))
  && (MpxyShmemSizeOnHart(0) == MpxyShmemSizeOnHart(0, old_s))
}
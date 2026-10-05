pub open spec fn drtm_version_spec(result: Result<(), NotSupported>, old_s: S, new_s: S) -> bool {
  (!DrtmIsSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (result.is_Ok() ==> result[31] == 0)
  && (result.is_Ok() ==> result[30:16] == 1)
  && (result.is_Ok() ==> result[15:0] == 4)
  && ((DrtmIsSupported(old_s))
    ==> result.is_Ok())
}
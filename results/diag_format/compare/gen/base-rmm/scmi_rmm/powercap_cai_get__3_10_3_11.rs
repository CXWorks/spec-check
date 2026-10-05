pub open spec fn powercap_cai_get__3_10_3_11_spec(result: Int32, cai: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_FOUND))
    && (!IsRequestSupported(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> cai == EnforcedCai(old_s, domain_id, cpli))
    && (ResultEqual(result, SUCCESS) ==> new_s == old_s)
}
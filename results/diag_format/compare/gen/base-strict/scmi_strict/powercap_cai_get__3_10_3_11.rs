pub open spec fn powercap_cai_get__3_10_3_11_spec(result: Int32, cai: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_FOUND))
    && (!DomainSupportsCpc(old_s, domain_id) && cpli != 0 ==> ResultEqual(result, NOT_FOUND))
    && (!IsCaiGetSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, SUCCESS) ==> cai == CurrentCai(old_s, domain_id, cpli))
    && (ResultEqual(result, SUCCESS) ==> old_s == new_s)
}
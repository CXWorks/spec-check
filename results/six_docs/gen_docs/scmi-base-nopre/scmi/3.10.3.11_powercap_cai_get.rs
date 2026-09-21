pub open spec fn powercap_cai_get_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (old_s.powercap_domains.len() as int == 0 || old_s.powercap_domains.len() as int > 0 && (old_s.powercap_domains[old_s.domain_id as usize].cpli as int != old_s.cpli as int)))
    && (result == RSI_ERROR_STATE ==> (old_s.powercap_domains.len() as int == 0 || old_s.powercap_domains.len() as int > 0 && (old_s.powercap_domains[old_s.domain_id as usize].cpli as int != old_s.cpli as int)))
    && (result == RSI_SUCCESS ==> (old_s.powercap_domains.len() as int > 0 && old_s.domain_id as int < old_s.powercap_domains.len() as int && old_s.powercap_domains[old_s.domain_id as usize].cpli as int == old_s.cpli as int))
    && (result == RSI_SUCCESS ==> (new_s.powercap_domains.len() as int == old_s.powercap_domains.len() as int))
    && (result == RSI_SUCCESS ==> (new_s.domain_id as int == old_s.domain_id as int))
    && (result == RSI_SUCCESS ==> (new_s.cpli as int == old_s.cpli as int))
    && (result == RSI_SUCCESS ==> (new_s.powercap_domains[old_s.domain_id as usize].cai as int == old_s.powercap_domains[old_s.domain_id as usize].cai as int))
}
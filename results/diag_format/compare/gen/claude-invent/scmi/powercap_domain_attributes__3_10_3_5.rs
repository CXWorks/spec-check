pub open spec fn powercap_domain_attributes__3_10_3_5_spec(domain_id: u32, status: i32, attributes: u32, name: Seq<u8>, min_mai: u32, max_mai: u32, mai_step: u32, min_power_cap: u32, max_power_cap: u32, power_cap_step: u32, sustainable_power: u32, accuracy: u32, parent_id: u32, min_cai: u32, max_cai: u32, cai_step: u32, old_s: S, new_s: S) -> bool {
    (!PowercapDomainExists(old_s, domain_id & 0xFFFFu32) ==> status == NOT_FOUND)
    && (PowercapDomainExists(old_s, domain_id & 0xFFFFu32) ==> (
        status == SUCCESS
        && (((attributes >> 27u32) & 1u32) == 0u32 ==> ((attributes >> 26u32) & 1u32) != 0u32)
        && (((attributes >> 26u32) & 1u32) == 0u32 ==> ((attributes >> 27u32) & 1u32) != 0u32)
        && ((attributes >> 23u32) & 3u32) != 3u32
        && (((attributes >> 27u32) & 1u32) == 1u32 ==> ((attributes >> 15u32) & 0xFu32) > 0u32)
        && (attributes & 0x7FFFu32) == 0u32
        && name.len() == 16
        && (exists|i: int| 0 <= i < 16 && name[i] == 0u8)
        && (min_mai != max_mai ==> mai_step != 0u32)
        && min_power_cap != 0u32
        && max_power_cap != 0u32
        && (min_power_cap != max_power_cap ==> power_cap_step != 0u32)
        && (min_cai != max_cai ==> cai_step != 0u32)
    ))
    && new_s == old_s
}

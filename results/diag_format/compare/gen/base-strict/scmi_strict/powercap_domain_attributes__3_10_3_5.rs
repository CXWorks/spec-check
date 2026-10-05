pub open spec fn powercap_domain_attributes__3_10_3_5_spec(result: Int32, attributes: UInt32, name: UInt8[16], min_mai: UInt32, max_mai: UInt32, mai_step: UInt32, min_power_cap: UInt32, max_power_cap: UInt32, power_cap_step: UInt32, sustainable_power: UInt32, accuracy: UInt32, parent_id: UInt32, min_cai: UInt32, max_cai: UInt32, cai_step: UInt32, old_s: S, new_s: S) -> bool {
    (!PowercapDomainExists(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (
        ResultEqual(result, SUCCESS)
        && Bits64(attributes as int, 31, 31) == MaiChangeNotifySupported(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int)
        && Bits64(attributes as int, 30, 30) == PowerMeasChangeNotifySupported(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int)
        && Bits64(attributes as int, 29, 29) == AsyncPowerCapSetSupported(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int)
        && Bits64(attributes as int, 28, 28) == (DomainNameLength(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int) > 16)
        && Bits64(attributes as int, 27, 27) == PowerCapConfigSupported(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int)
        && Bits64(attributes as int, 26, 26) == PowerMonitoringSupported(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int)
        && (Bits64(attributes as int, 27, 27) == 1 || Bits64(attributes as int, 26, 26) == 1)
        && Bits64(attributes as int, 25, 25) == MaiConfigSupported(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int)
        && Bits64(attributes as int, 24, 23) <= 2
        && Bits64(attributes as int, 22, 22) == HasFastChannel(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int)
        && Bits64(attributes as int, 21, 21) == PowerCapChangeNotifySupported(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int)
        && Bits64(attributes as int, 20, 20) == CaiConfigSupported(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int)
        && Bits64(attributes as int, 19, 19) == CaiChangeNotifySupported(Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int)
        && (Bits64(attributes as int, 27, 27) == 1 ==> Bits64(attributes as int, 18, 15) > 0)
        && Bits64(attributes as int, 14, 0) == 0
        && NameMatchesDomain(name, Bits64(old_s.cmd_input_domain_id as int, 15, 0) as int, Bits64(attributes as int, 28, 28))
        && (min_mai != max_mai ==> mai_step != 0)
        && min_power_cap != 0
        && max_power_cap != 0
        && (min_power_cap != max_power_cap ==> power_cap_step != 0)
        && (min_cai != max_cai ==> cai_step != 0)
        && (parent_id == 0xFFFFFFFF || PowercapDomainExists(parent_id))
    ))
    && (old_s == new_s)
}
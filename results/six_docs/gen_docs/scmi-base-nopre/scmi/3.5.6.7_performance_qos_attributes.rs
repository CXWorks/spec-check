pub open spec fn 3.5.6.7_performance_qos_attributes_spec(result: (int32, uint32, [uint8; 16]), old_s: S, new_s: S) -> bool {
    let (status, qos_attribute_1, name) = result;
    let domain_id: uint32 = old_s.cmd_input_0;
    let capability: uint32 = old_s.cmd_input_1;
    (status == 0) ==> (qos_attribute_1 == 0 && name[0] == 0)
    && (status != 0) ==> (
        (status == 1) ==> (
            (domain_id == 0 || capability == 0) ==> false
        )
        && (
            (domain_id != 0 && capability != 0) ==> (
                (qos_attribute_1 != 0) && (name[0] != 0)
            )
        )
    )
    && (status == 2) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 3) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 4) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 5) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 6) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 7) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 8) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 9) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 10) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 11) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 12) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 13) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 14) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 15) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 16) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 17) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 18) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 19) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 20) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 21) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 22) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 23) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 24) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 25) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 26) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 27) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 28) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 29) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 30) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 31) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 32) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 33) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 34) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 35) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 36) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 37) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 38) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 39) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 40) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 41) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 42) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 43) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 44) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 45) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 46) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 47) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 48) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 49) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 50) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 51) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 52) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 53) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 54) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 55) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 56) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 57) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 58) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 59) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 60) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 61) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 62) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 63) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 64) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 65) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 66) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 67) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 68) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 69) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 70) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 71) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 72) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 73) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 74) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 75) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 76) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 77) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 78) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 79) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 80) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 81) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 82) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 83) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 84) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 85) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 86) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 87) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 88) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 89) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 90) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 91) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 92) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 93) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 94) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 95) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 96) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 97) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 98) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 99) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 100) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 101) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 102) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 103) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 104) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 105) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 106) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 107) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 108) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 109) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 110) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 111) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 112) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 113) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 114) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 115) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 116) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 117) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 118) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 119) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 120) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 121) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 122) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 123) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 124) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 125) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 126) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 127) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 128) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 129) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 130) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 131) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 132) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 133) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 134) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 135) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 136) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 137) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 138) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 139) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 140) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 141) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 142) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 143) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 144) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 145) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 146) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 147) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 148) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 149) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 150) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 151) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 152) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 153) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 154) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 155) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 156) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 157) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 158) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >> 8) != 0)
    )
    && (status == 159) ==> (
        (capability & (capability - 1) == 0) && (capability & (capability >> 8) == 0)
    )
    && (status == 160) ==> (
        (domain_id == 0 || capability == 0)
    )
    && (status == 161) ==> (
        (domain_id != 0 && capability != 0)
    )
    && (status == 162) ==> (
        (capability & (capability - 1) != 0) || (capability & (capability >>
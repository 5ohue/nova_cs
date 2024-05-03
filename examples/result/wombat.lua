local colors = {
    fg = "#E5E1D8",
    bg = "#1F1F1F",
    bright = {
        red     = "#F99F92",
        yellow  = "#F2E9BF",
        green   = "#E3F7A1",
        cyan    = "#C2FEFA",
        blue    = "#B3D2FF",
        magenta = "#E5BDFF",
    },
    normal = {
        red     = "#F7786D",
        yellow  = "#EFDFAC",
        green   = "#BDE97C",
        cyan    = "#90FDF8",
        blue    = "#6EBAF8",
        magenta = "#EF88FF",
    },
    bg_normal_25 = {
        red     = "#553533",
        yellow  = "#534F42",
        green   = "#475236",
        cyan    = "#3B5755",
        blue    = "#334655",
        magenta = "#533957",
    },
    bg_normal_50 = {
        red     = "#8B4C46",
        yellow  = "#877F66",
        green   = "#6E844E",
        cyan    = "#588E8C",
        blue    = "#476D8C",
        magenta = "#87548F",
    },
    bg_normal_75 = {
        red     = "#C1625A",
        yellow  = "#BBAF89",
        green   = "#96B765",
        cyan    = "#74C6C2",
        blue    = "#5A93C2",
        magenta = "#BB6EC7",
    },
    normal_bright_25 = {
        red     = "#F88C80",
        yellow  = "#F1E4B6",
        green   = "#D0F08F",
        cyan    = "#A9FEF9",
        blue    = "#91C6FC",
        magenta = "#EAA3FF",
    },
    normal_bright_50 = {
        red     = "#F88C80",
        yellow  = "#F1E4B6",
        green   = "#D0F08F",
        cyan    = "#A9FEF9",
        blue    = "#91C6FC",
        magenta = "#EAA3FF",
    },
    normal_bright_75 = {
        red     = "#F88C80",
        yellow  = "#F1E4B6",
        green   = "#D0F08F",
        cyan    = "#A9FEF9",
        blue    = "#91C6FC",
        magenta = "#EAA3FF",
    },
    bright_fg_25 = {
        red     = "#EFC0B5",
        yellow  = "#ECE5CC",
        green   = "#E4ECBD",
        cyan    = "#D4F0E9",
        blue    = "#CCDAEC",
        magenta = "#E5CFEC",
    },
    bright_fg_50 = {
        red     = "#EFC0B5",
        yellow  = "#ECE5CC",
        green   = "#E4ECBD",
        cyan    = "#D4F0E9",
        blue    = "#CCDAEC",
        magenta = "#E5CFEC",
    },
    bright_fg_75 = {
        red     = "#EFC0B5",
        yellow  = "#ECE5CC",
        green   = "#E4ECBD",
        cyan    = "#D4F0E9",
        blue    = "#CCDAEC",
        magenta = "#E5CFEC",
    },
    mixed_bright = {
        red_yellow   = "#F0C8A4",
        yellow_green = "#EEEFAC",
        green_cyan   = "#CDFBDD",
        cyan_blue    = "#AEEBFE",
        blue_magenta = "#CCC8FF",
        magenta_red  = "#FCA7D6",
    },
    mixed_normal = {
        red_yellow   = "#EDB381",
        yellow_green = "#DDE393",
        green_cyan   = "#81FABF",
        cyan_blue    = "#60DFFA",
        blue_magenta = "#B4A7FB",
        magenta_red  = "#FB7BC0",
    },
    mixed_bg_normal_25 = {
        red_yellow   = "#51443A",
        yellow_green = "#4F503C",
        green_cyan   = "#385645",
        cyan_blue    = "#374F54",
        blue_magenta = "#423C74",
        magenta_red  = "#563646",
    },
    mixed_bg_normal_50 = {
        red_yellow   = "#836955",
        yellow_green = "#7E815A",
        green_cyan   = "#538C6F",
        cyan_blue    = "#4F7E8A",
        blue_magenta = "#695CB2",
        magenta_red  = "#8E4D6F",
    },
    mixed_bg_normal_75 = {
        red_yellow   = "#B58F70",
        yellow_green = "#AEB278",
        green_cyan   = "#6CC397",
        cyan_blue    = "#66ADBE",
        blue_magenta = "#8F82D1",
        magenta_red  = "#C66398",
    },
    mixed_normal_bright_25 = {
        red_yellow   = "#EFBD93",
        yellow_green = "#E6E99E",
        green_cyan   = "#ACFBCD",
        cyan_blue    = "#88E5FD",
        blue_magenta = "#BFB7FD",
        magenta_red  = "#FB92CA",
    },
    mixed_normal_bright_50 = {
        red_yellow   = "#EFBD93",
        yellow_green = "#E6E99E",
        green_cyan   = "#ACFBCD",
        cyan_blue    = "#88E5FD",
        blue_magenta = "#BFB7FD",
        magenta_red  = "#FB92CA",
    },
    mixed_normal_bright_75 = {
        red_yellow   = "#EFBD93",
        yellow_green = "#E6E99E",
        green_cyan   = "#ACFBCD",
        cyan_blue    = "#88E5FD",
        blue_magenta = "#BFB7FD",
        magenta_red  = "#FB92CA",
    },
    mixed_bright_fg_25 = {
        red_yellow   = "#EBD4BF",
        yellow_green = "#EAE8C3",
        green_cyan   = "#D3F0D7",
        cyan_blue    = "#C8E7ED",
        blue_magenta = "#D7D5EC",
        magenta_red  = "#EDC6D6",
    },
    mixed_bright_fg_50 = {
        red_yellow   = "#EBD4BF",
        yellow_green = "#EAE8C3",
        green_cyan   = "#D3F0D7",
        cyan_blue    = "#C8E7ED",
        blue_magenta = "#D7D5EC",
        magenta_red  = "#EDC6D6",
    },
    mixed_bright_fg_75 = {
        red_yellow   = "#EBD4BF",
        yellow_green = "#EAE8C3",
        green_cyan   = "#D3F0D7",
        cyan_blue    = "#C8E7ED",
        blue_magenta = "#D7D5EC",
        magenta_red  = "#EDC6D6",
    },

    gray_n05 = "#151516",
    gray_n04 = "#171718",
    gray_n03 = "#191919",
    gray_n02 = "#1B1B1B",
    gray_n01 = "#1D1D1D",
    gray_00 = "#1F1F1F",
    gray_01 = "#212121",
    gray_02 = "#232323",
    gray_03 = "#252525",
    gray_04 = "#272726",
    gray_05 = "#292928",
    gray_06 = "#2B2B2A",
    gray_07 = "#2D2D2C",
    gray_08 = "#2F2F2E",
    gray_09 = "#313030",
    gray_10 = "#333232",
    gray_11 = "#353433",
    gray_12 = "#373635",
    gray_13 = "#393837",
    gray_14 = "#3B3A39",
    gray_15 = "#3D3C3B",
    gray_16 = "#3F3E3D",
    gray_17 = "#41403E",
    gray_18 = "#434240",
    gray_19 = "#454442",
    gray_20 = "#474644",
    gray_21 = "#494846",
    gray_22 = "#4B4A48",
    gray_23 = "#4D4C4A",
    gray_24 = "#4F4E4B",
    gray_25 = "#51504D",
    gray_26 = "#52514F",
    gray_27 = "#545351",
    gray_28 = "#565553",
    gray_29 = "#585755",
    gray_30 = "#5A5957",
    gray_31 = "#5C5B58",
    gray_32 = "#5E5D5A",
    gray_33 = "#605F5C",
    gray_34 = "#62615E",
    gray_35 = "#646360",
    gray_36 = "#666562",
    gray_37 = "#686763",
    gray_38 = "#6A6965",
    gray_39 = "#6C6B67",
    gray_40 = "#6E6D69",
    gray_41 = "#706F6B",
    gray_42 = "#72706D",
    gray_43 = "#74726F",
    gray_44 = "#767470",
    gray_45 = "#787672",
    gray_46 = "#7A7874",
    gray_47 = "#7C7A76",
    gray_48 = "#7E7C78",
    gray_49 = "#807E7A",
    gray_50 = "#82807C",
    gray_51 = "#84827D",
    gray_52 = "#86847F",
    gray_53 = "#888681",
    gray_54 = "#8A8883",
    gray_55 = "#8C8A85",
    gray_56 = "#8E8C87",
    gray_57 = "#908E88",
    gray_58 = "#92908A",
    gray_59 = "#94918C",
    gray_60 = "#96938E",
    gray_61 = "#989590",
    gray_62 = "#9A9792",
    gray_63 = "#9C9994",
    gray_64 = "#9E9B95",
    gray_65 = "#A09D97",
    gray_66 = "#A29F99",
    gray_67 = "#A4A19B",
    gray_68 = "#A6A39D",
    gray_69 = "#A8A59F",
    gray_70 = "#AAA7A1",
    gray_71 = "#ACA9A2",
    gray_72 = "#AEABA4",
    gray_73 = "#B0ADA6",
    gray_74 = "#B2AFA8",
    gray_75 = "#B4B1AA",
    gray_76 = "#B5B2AC",
    gray_77 = "#B7B4AD",
    gray_78 = "#B9B6AF",
    gray_79 = "#BBB8B1",
    gray_80 = "#BDBAB3",
    gray_81 = "#BFBCB5",
    gray_82 = "#C1BEB7",
    gray_83 = "#C3C0B9",
    gray_84 = "#C5C2BA",
    gray_85 = "#C7C4BC",
    gray_86 = "#C9C6BE",
    gray_87 = "#CBC8C0",
    gray_88 = "#CDCAC2",
    gray_89 = "#CFCCC4",
    gray_90 = "#D1CEC6",
    gray_91 = "#D3D0C7",
    gray_92 = "#D5D1C9",
    gray_93 = "#D7D3CB",
    gray_94 = "#D9D5CD",
    gray_95 = "#DBD7CF",
    gray_96 = "#DDD9D1",
    gray_97 = "#DFDBD2",
    gray_98 = "#E1DDD4",
}

return colors
use crate::fields::FieldContent;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeZone {
    Adelaide,
    Alberta,
    Almaty,
    Amsterdam,
    Athens,
    Auckland,
    AustraliaLh,
    Automatic,
    Bangkok,
    Barcelona,
    Berlin,
    Boise,
    Bombay,
    Boston,
    Brasilia,
    Brisbane,
    BritishColumbia,
    Brussels,
    Budapest,
    Cairo,
    CapeTown,
    CapeVerdeIs,
    Chicago,
    Chita,
    Copenhagen,
    Dallas,
    Darwin,
    Denver,
    Dublin,
    Ekaterinburg,
    Eniwetok,
    EuropeCentralCet,
    EuropeEasternEet,
    EuropeWesternWet,
    Fiji,
    Helsinki,
    HongKong,
    Iceland,
    Irkutsk,
    Islamabad,
    Jakarta,
    Kabul,
    Kaliningrad,
    KansasCity,
    Kathmandu,
    Krasnoyarsk,
    Lagos,
    LasVegas,
    Lisbon,
    London,
    LosAngeles,
    Madrid,
    Magadan,
    Manitoba,
    Manual,
    MexicoCentral,
    MexicoMountain,
    MexicoPacific,
    Miami,
    MidAtlantic,
    Minneapolis,
    Moscow,
    Munich,
    Muscat,
    NewOrleans,
    NewYork,
    Newfoundland,
    Novosibirsk,
    Ontario,
    Oslo,
    Other,
    Paris,
    Perth,
    PetropavlovskKamchatskiy,
    Phoenix,
    Prague,
    Quebec,
    Reykjavik,
    Riyahd,
    Rome,
    Samara,
    Samoa,
    SantaFe,
    Santiago,
    Saskatchewan,
    Seattle,
    Stockholm,
    Sydney,
    Tasmania,
    Tehran,
    Tokyo,
    UsAlaska,
    UsArizona,
    UsAtlantic,
    UsCentral,
    UsEastern,
    UsHawaii,
    UsMountain,
    UsPacific,
    Venezuela,
    Vienna,
    Vladivostok,
    Warsaw,
    WashingtonDc,
    Winkhoek,
    Zurich,
    UnknownValue(u64),
}

impl From<FieldContent> for TimeZone {
    fn from(field: FieldContent) -> Self {
        if let FieldContent::Enum(enum_value) = field {
            match enum_value {
                0 => TimeZone::Almaty,
                1 => TimeZone::Bangkok,
                2 => TimeZone::Bombay,
                3 => TimeZone::Brasilia,
                4 => TimeZone::Cairo,
                5 => TimeZone::CapeVerdeIs,
                6 => TimeZone::Darwin,
                7 => TimeZone::Eniwetok,
                8 => TimeZone::Fiji,
                9 => TimeZone::HongKong,
                10 => TimeZone::Islamabad,
                11 => TimeZone::Kabul,
                12 => TimeZone::Magadan,
                13 => TimeZone::MidAtlantic,
                14 => TimeZone::Moscow,
                15 => TimeZone::Muscat,
                16 => TimeZone::Newfoundland,
                17 => TimeZone::Samoa,
                18 => TimeZone::Sydney,
                19 => TimeZone::Tehran,
                20 => TimeZone::Tokyo,
                21 => TimeZone::UsAlaska,
                22 => TimeZone::UsAtlantic,
                23 => TimeZone::UsCentral,
                24 => TimeZone::UsEastern,
                25 => TimeZone::UsHawaii,
                26 => TimeZone::UsMountain,
                27 => TimeZone::UsPacific,
                28 => TimeZone::Other,
                29 => TimeZone::Auckland,
                30 => TimeZone::Kathmandu,
                31 => TimeZone::EuropeWesternWet,
                32 => TimeZone::EuropeCentralCet,
                33 => TimeZone::EuropeEasternEet,
                34 => TimeZone::Jakarta,
                35 => TimeZone::Perth,
                36 => TimeZone::Adelaide,
                37 => TimeZone::Brisbane,
                38 => TimeZone::Tasmania,
                39 => TimeZone::Iceland,
                40 => TimeZone::Amsterdam,
                41 => TimeZone::Athens,
                42 => TimeZone::Barcelona,
                43 => TimeZone::Berlin,
                44 => TimeZone::Brussels,
                45 => TimeZone::Budapest,
                46 => TimeZone::Copenhagen,
                47 => TimeZone::Dublin,
                48 => TimeZone::Helsinki,
                49 => TimeZone::Lisbon,
                50 => TimeZone::London,
                51 => TimeZone::Madrid,
                52 => TimeZone::Munich,
                53 => TimeZone::Oslo,
                54 => TimeZone::Paris,
                55 => TimeZone::Prague,
                56 => TimeZone::Reykjavik,
                57 => TimeZone::Rome,
                58 => TimeZone::Stockholm,
                59 => TimeZone::Vienna,
                60 => TimeZone::Warsaw,
                61 => TimeZone::Zurich,
                62 => TimeZone::Quebec,
                63 => TimeZone::Ontario,
                64 => TimeZone::Manitoba,
                65 => TimeZone::Saskatchewan,
                66 => TimeZone::Alberta,
                67 => TimeZone::BritishColumbia,
                68 => TimeZone::Boise,
                69 => TimeZone::Boston,
                70 => TimeZone::Chicago,
                71 => TimeZone::Dallas,
                72 => TimeZone::Denver,
                73 => TimeZone::KansasCity,
                74 => TimeZone::LasVegas,
                75 => TimeZone::LosAngeles,
                76 => TimeZone::Miami,
                77 => TimeZone::Minneapolis,
                78 => TimeZone::NewYork,
                79 => TimeZone::NewOrleans,
                80 => TimeZone::Phoenix,
                81 => TimeZone::SantaFe,
                82 => TimeZone::Seattle,
                83 => TimeZone::WashingtonDc,
                84 => TimeZone::UsArizona,
                85 => TimeZone::Chita,
                86 => TimeZone::Ekaterinburg,
                87 => TimeZone::Irkutsk,
                88 => TimeZone::Kaliningrad,
                89 => TimeZone::Krasnoyarsk,
                90 => TimeZone::Novosibirsk,
                91 => TimeZone::PetropavlovskKamchatskiy,
                92 => TimeZone::Samara,
                93 => TimeZone::Vladivostok,
                94 => TimeZone::MexicoCentral,
                95 => TimeZone::MexicoMountain,
                96 => TimeZone::MexicoPacific,
                97 => TimeZone::CapeTown,
                98 => TimeZone::Winkhoek,
                99 => TimeZone::Lagos,
                100 => TimeZone::Riyahd,
                101 => TimeZone::Venezuela,
                102 => TimeZone::AustraliaLh,
                103 => TimeZone::Santiago,
                253 => TimeZone::Manual,
                254 => TimeZone::Automatic,
                n => TimeZone::UnknownValue(n as u64),
            }
        } else {
          panic!("can't convert TimeZone to {:?}", field);
        }
    }
}

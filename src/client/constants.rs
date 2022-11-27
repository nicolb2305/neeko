#[derive(PartialEq, Hash, Clone)]
pub enum Region {
    BR,
    EUNE,
    EUW,
    JP,
    KR,
    LAN,
    LAS,
    NA,
    OCE,
    RU,
    TR
}

impl Region {
    pub fn to_short_region(&self) -> &'static str {
        match self {
            Region::BR => "br1",
            Region::EUNE => "eun1",
            Region::EUW => "euw1",
            Region::JP => "jp1",
            Region::KR => "kr",
            Region::LAN => "la1",
            Region::LAS => "la2",
            Region::NA => "na1",
            Region::OCE => "oc1",
            Region::RU => "ru",
            Region::TR => "tr1"
        }
    }
    
    pub fn to_long_region(&self) -> &'static str {
        match self {
            Region::BR | Region::LAN | Region::LAS | Region::NA => "americas",
            Region::EUNE | Region::EUW | Region::RU | Region::TR => "europe",
            Region::JP | Region::KR | Region::OCE => "sea",
        }
    }
}
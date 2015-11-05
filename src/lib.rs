#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SteamId(u64);

impl SteamId {
    pub fn from_u64(u: u64) -> SteamId {
        SteamId(u)
    }

    pub fn to_u64(self) -> u64 {
        self.0
    }

    pub fn get_account_id(self) -> u32 {
        (self.0 & 0xFFFFFFFF) as u32 
    }

    pub fn get_instance(self) -> u32 {
        ((self.0 >> 32) & 0xFFFFF) as u32
    }

    pub fn get_type(self) -> Option<AccountType> {
        use AccountType::*;

        let int_rep = (self.0 >> 52) & 0xF;
        match int_rep {
            0 => Some(Invalid),
            1 => Some(Individual),
            2 => Some(Multiseat),
            3 => Some(GameServer),
            4 => Some(AnonGameServer),
            5 => Some(Pending),
            6 => Some(ContentServer),
            7 => Some(Clan),
            8 => Some(Chat),
            9 => Some(SuperSeeder),
            10 => Some(AnonUser),
            _ => None
        }
    }

    pub fn get_universe(self) -> Option<Universe> {
        use Universe::*;

        let int_rep = (self.0 >> 56) & 0xFF;
        match int_rep {
            0 => Some(Invalid),
            1 => Some(Public),
            2 => Some(Beta),
            3 => Some(Internal),
            4 => Some(Dev),
            _ => None
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AccountType {
    Invalid = 0,
    Individual = 1,
    Multiseat = 2,
    GameServer = 3,
    AnonGameServer = 4,
    Pending = 5,
    ContentServer = 6,
    Clan = 7,
    Chat = 8,
    SuperSeeder = 9,
    AnonUser = 10
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Universe {
    Invalid = 0,
    Public = 1,
    Beta = 2,
    Internal = 3,
    Dev = 4
}

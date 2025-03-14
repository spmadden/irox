// SPDX-License-Identifier: MIT
// Copyright 2025 IROX Contributors
//

//!
//! Takes a hash value and generates a deterministic set of six english words
//! to represent that hash.  Loosely inspired by RFC2289.

use crate::hash::{HashDigest, MD5, SHA1};

pub fn fold_hash_to_u64<const N: usize>(inp: &[u8; N]) -> u64 {
    if N < 8 {
        return fold_hash_to_u64(&SHA1::new().hash(inp));
    }
    let mut out = 0u64;
    let mut shift = 0;
    for v in inp {
        out ^= (*v as u64) << shift;
        shift += 8;
        if shift >= 64 {
            shift = 0;
        }
    }
    out
}

fn checksum(value: u64) -> u64 {
    let mut out = 0u64;
    let mut shift = 0;
    while shift < 64 {
        out += (value >> shift) & 0x3;
        shift += 2;
    }
    out & 0x3
}

///
/// Hashes the data with [`MD5`] then uses the hash as input into [`get_six_words_by_hash`]
pub fn get_six_words_for_md5(data: &[u8]) -> [&'static str; 6] {
    let hash = MD5::default().hash(data).to_be_bytes();
    get_six_words_by_hash(&hash)
}

///
/// Hashes the data with [`SHA1`] then uses the hash as input into [`get_six_words_by_hash`]
pub fn get_six_words_for_sha1(data: &[u8]) -> [&'static str; 6] {
    let hash = SHA1::new().hash(data);
    get_six_words_by_hash(&hash)
}

///
/// Given the provided data, hashes it using the provided algorithm, then uses
/// the has as input into [`get_six_words_by_hash`]
pub fn get_six_words_for_data<HASH: HashDigest<R, N>, const N: usize, const R: usize>(
    data: &[u8],
) -> [&'static str; 6] {
    let h = HASH::default().hash(data);
    get_six_words_by_hash(&h)
}

///
/// Given a particular hash, folds that hash into 64 bits and uses it to call [`get_six_words_by_value`].
///
/// The folding operation is one of three steps:
/// * If the length `N` is less than 8: hashes that value with [`SHA1`], then folds the value
/// * If the length `N` is exactly 8, just passes that value as a [`u64`]
/// * If the length `N` is greater than 8, xors each group of 8 bytes together.
pub fn get_six_words_by_hash<const N: usize>(hash: &[u8; N]) -> [&'static str; 6] {
    let val = fold_hash_to_u64(hash);
    get_six_words_by_value(val)
}

///
/// Given a particular [`u64`] value (like from a hash) returns a unique and
/// deterministic set of six words, representing that value.
///
/// Inspired by RFC2289, breaks the u64 into 6x 11-bit blocks and a single 2-bit
/// checksum (for a total of 66 bits), and uses each 11-bit block as an index
/// into the [`WORDLIST`]
pub fn get_six_words_by_value(value: u64) -> [&'static str; 6] {
    let cksum = checksum(value);
    let a = value >> 53;
    let b = (value >> 42) & 0x7FF;
    let c = (value >> 31) & 0x7FF;
    let d = (value >> 20) & 0x7FF;
    let e = (value >> 9) & 0x7FF;
    let f = (value << 2) & 0x7FF | cksum;
    let a = WORDLIST.get(a as usize).copied().unwrap_or_default();
    let b = WORDLIST.get(b as usize).copied().unwrap_or_default();
    let c = WORDLIST.get(c as usize).copied().unwrap_or_default();
    let d = WORDLIST.get(d as usize).copied().unwrap_or_default();
    let e = WORDLIST.get(e as usize).copied().unwrap_or_default();
    let f = WORDLIST.get(f as usize).copied().unwrap_or_default();

    [a, b, c, d, e, f]
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SixWordsError<'a> {
    InvalidWord(&'a str),
    InvalidChecksum,
}

///
/// Takes the provided six words, and returns the associated 64 bit value that would
/// have generated them.
pub fn get_value_for_six_words<'a>(words: &'a [&'a str; 6]) -> Result<u64, SixWordsError<'a>> {
    let mut values = [0u64; 6];
    for (v, word) in values.iter_mut().zip(words) {
        let w = word.to_ascii_uppercase();
        let w = w.as_str();
        if let Some(idx) = WORDLIST
            .iter()
            .enumerate()
            .find_map(|(idx, word)| (*word == w).then_some(idx as u64))
        {
            *v = idx;
        } else {
            return Err(SixWordsError::InvalidWord(word));
        }
    }
    let [a, b, c, d, e, f] = values;
    let mut out = 0u64;
    out |= a << 53;
    out |= b << 42;
    out |= c << 31;
    out |= d << 20;
    out |= e << 9;
    out |= f >> 2;

    let cksum = checksum(out);
    if cksum != (f & 0x3) {
        return Err(SixWordsError::InvalidChecksum);
    }

    Ok(out)
}

///
/// The wordlist from RFC2289, 2048 words, or 11 bits of data per word.
#[rustfmt::skip]
pub static WORDLIST : [&str;2048] = [
    "A",     "ABE",   "ACE",   "ACT",   "AD",    "ADA",   "ADD",
    "AGO",   "AID",   "AIM",   "AIR",   "ALL",   "ALP",   "AM",    "AMY",
    "AN",    "ANA",   "AND",   "ANN",   "ANT",   "ANY",   "APE",   "APS",
    "APT",   "ARC",   "ARE",   "ARK",   "ARM",   "ART",   "AS",    "ASH",
    "ASK",   "AT",    "ATE",   "AUG",   "AUK",   "AVE",   "AWE",   "AWK",
    "AWL",   "AWN",   "AX",   "AYE",   "BAD",   "BAG",   "BAH",   "BAM",
    "BAN",   "BAR",   "BAT",   "BAY",   "BE",    "BED",   "BEE",   "BEG",
    "BEN",   "BET",   "BEY",   "BIB",   "BID",   "BIG",   "BIN",   "BIT",
    "BOB",   "BOG",   "BON",   "BOO",   "BOP",   "BOW",   "BOY",   "BUB",
    "BUD",   "BUG",   "BUM",   "BUN",   "BUS",   "BUT",   "BUY",   "BY",
    "BYE",   "CAB",   "CAL",   "CAM",   "CAN",   "CAP",   "CAR",   "CAT",
    "CAW",   "COD",   "COG",   "COL",   "CON",   "COO",   "COP",   "COT",
    "COW",   "COY",   "CRY",   "CUB",   "CUE",   "CUP",   "CUR",   "CUT",
    "DAB",   "DAD",   "DAM",   "DAN",   "DAR",   "DAY",   "DEE",   "DEL",
    "DEN",   "DES",   "DEW",   "DID",   "DIE",   "DIG",   "DIN",   "DIP",
    "DO",    "DOE",   "DOG",   "DON",   "DOT",   "DOW",   "DRY",   "DUB",
    "DUD",   "DUE",   "DUG",   "DUN",   "EAR",   "EAT",   "ED",    "EEL",
    "EGG",   "EGO",   "ELI",   "ELK",   "ELM",   "ELY",   "EM",    "END",
    "EST",   "ETC",   "EVA",   "EVE",   "EWE",   "EYE",   "FAD",   "FAN",
    "FAR",   "FAT",   "FAY",   "FED",   "FEE",   "FEW",   "FIB",   "FIG",
    "FIN",   "FIR",   "FIT",   "FLO",   "FLY",   "FOE",   "FOG",   "FOR",
    "FRY",   "FUM",   "FUN",   "FUR",   "GAB",   "GAD",   "GAG",   "GAL",
    "GAM",   "GAP",   "GAS",   "GAY",   "GEE",   "GEL",   "GEM",   "GET",
    "GIG",   "GIL",   "GIN",   "GO",    "GOT",   "GUM",   "GUN",   "GUS",
    "GUT",   "GUY",   "GYM",   "GYP",   "HA",    "HAD",   "HAL",   "HAM",
    "HAN",   "HAP",   "HAS",   "HAT",   "HAW",   "HAY",   "HE",    "HEM",
    "HEN",   "HER",   "HEW",   "HEY",   "HI",    "HID",   "HIM",   "HIP",
    "HIS",   "HIT",   "HO",   "HOB",   "HOC",   "HOE",   "HOG",   "HOP",
    "HOT",   "HOW",   "HUB",   "HUE",   "HUG",   "HUH",   "HUM",   "HUT",
    "I",     "ICY",   "IDA",   "IF",    "IKE",   "ILL",   "INK",   "INN",
    "IO",    "ION",   "IQ",   "IRA",   "IRE",   "IRK",   "IS",    "IT",
    "ITS",   "IVY",   "JAB",   "JAG",   "JAM",   "JAN",   "JAR",   "JAW",
    "JAY",   "JET",   "JIG",   "JIM",   "JO",    "JOB",   "JOE",   "JOG",
    "JOT",   "JOY",   "JUG",   "JUT",   "KAY",   "KEG",   "KEN",   "KEY",
    "KID",   "KIM",   "KIN",   "KIT",   "LA",    "LAB",   "LAC",   "LAD",
    "LAG",   "LAM",   "LAP",   "LAW",   "LAY",   "LEA",   "LED",   "LEE",
    "LEG",   "LEN",   "LEO",   "LET",   "LEW",   "LID",   "LIE",   "LIN",
    "LIP",   "LIT",   "LO",   "LOB",   "LOG",   "LOP",   "LOS",   "LOT",
    "LOU",   "LOW",   "LOY",   "LUG",   "LYE",   "MA",    "MAC",   "MAD",
    "MAE",   "MAN",   "MAO",   "MAP",   "MAT",   "MAW",   "MAY",   "ME",
    "MEG",   "MEL",   "MEN",   "MET",   "MEW",   "MID",   "MIN",   "MIT",
    "MOB",   "MOD",   "MOE",   "MOO",   "MOP",   "MOS",   "MOT",   "MOW",
    "MUD",   "MUG",   "MUM",   "MY",    "NAB",   "NAG",   "NAN",   "NAP",
    "NAT",   "NAY",   "NE",   "NED",   "NEE",   "NET",   "NEW",   "NIB",
    "NIL",   "NIP",   "NIT",   "NO",    "NOB",   "NOD",   "NON",   "NOR",
    "NOT",   "NOV",   "NOW",   "NU",    "NUN",   "NUT",   "O",     "OAF",
    "OAK",   "OAR",   "OAT",   "ODD",   "ODE",   "OF",    "OFF",   "OFT",
    "OH",    "OIL",   "OK",   "OLD",   "ON",    "ONE",   "OR",    "ORB",
    "ORE",   "ORR",   "OS",   "OTT",   "OUR",   "OUT",   "OVA",   "OW",
    "OWE",   "OWL",   "OWN",   "OX",    "PA",    "PAD",   "PAL",   "PAM",
    "PAN",   "PAP",   "PAR",   "PAT",   "PAW",   "PAY",   "PEA",   "PEG",
    "PEN",   "PEP",   "PER",   "PET",   "PEW",   "PHI",   "PI",    "PIE",
    "PIN",   "PIT",   "PLY",   "PO",    "POD",   "POE",   "POP",   "POT",
    "POW",   "PRO",   "PRY",   "PUB",   "PUG",   "PUN",   "PUP",   "PUT",
    "QUO",   "RAG",   "RAM",   "RAN",   "RAP",   "RAT",   "RAW",   "RAY",
    "REB",   "RED",   "REP",   "RET",   "RIB",   "RID",   "RIG",   "RIM",
    "RIO",   "RIP",   "ROB",   "ROD",   "ROE",   "RON",   "ROT",   "ROW",
    "ROY",   "RUB",   "RUE",   "RUG",   "RUM",   "RUN",   "RYE",   "SAC",
    "SAD",   "SAG",   "SAL",   "SAM",   "SAN",   "SAP",   "SAT",   "SAW",
    "SAY",   "SEA",   "SEC",   "SEE",   "SEN",   "SET",   "SEW",   "SHE",
    "SHY",   "SIN",   "SIP",   "SIR",   "SIS",   "SIT",   "SKI",   "SKY",
    "SLY",   "SO",    "SOB",   "SOD",   "SON",   "SOP",   "SOW",   "SOY",
    "SPA",   "SPY",   "SUB",   "SUD",   "SUE",   "SUM",   "SUN",   "SUP",
    "TAB",   "TAD",   "TAG",   "TAN",   "TAP",   "TAR",   "TEA",   "TED",
    "TEE",   "TEN",   "THE",   "THY",   "TIC",   "TIE",   "TIM",   "TIN",
    "TIP",   "TO",    "TOE",   "TOG",   "TOM",   "TON",   "TOO",   "TOP",
    "TOW",   "TOY",   "TRY",   "TUB",   "TUG",   "TUM",   "TUN",   "TWO",
    "UN",    "UP",    "US",   "USE",   "VAN",   "VAT",   "VET",   "VIE",
    "WAD",   "WAG",   "WAR",   "WAS",   "WAY",   "WE",    "WEB",   "WED",
    "WEE",   "WET",   "WHO",   "WHY",   "WIN",   "WIT",   "WOK",   "WON",
    "WOO",   "WOW",   "WRY",   "WU",    "YAM",   "YAP",   "YAW",   "YE",
    "YEA",   "YES",   "YET",   "YOU",   "ABED",  "ABEL",  "ABET",  "ABLE",
    "ABUT",  "ACHE",  "ACID",  "ACME",  "ACRE",  "ACTA",  "ACTS",  "ADAM",
    "ADDS",  "ADEN",  "AFAR",  "AFRO",  "AGEE",  "AHEM",  "AHOY",  "AIDA",
    "AIDE",  "AIDS",  "AIRY",  "AJAR",  "AKIN",  "ALAN",  "ALEC",  "ALGA",
    "ALIA",  "ALLY",  "ALMA",  "ALOE",  "ALSO",  "ALTO",  "ALUM",  "ALVA",
    "AMEN",  "AMES",  "AMID",  "AMMO",  "AMOK",  "AMOS",  "AMRA",  "ANDY",
    "ANEW",  "ANNA",  "ANNE",  "ANTE",  "ANTI",  "AQUA",  "ARAB",  "ARCH",
    "AREA",  "ARGO",  "ARID",  "ARMY",  "ARTS",  "ARTY",  "ASIA",  "ASKS",
    "ATOM",  "AUNT",  "AURA",  "AUTO",  "AVER",  "AVID",  "AVIS",  "AVON",
    "AVOW",  "AWAY",  "AWRY",  "BABE",  "BABY",  "BACH",  "BACK",  "BADE",
    "BAIL",  "BAIT",  "BAKE",  "BALD",  "BALE",  "BALI",  "BALK",  "BALL",
    "BALM",  "BAND",  "BANE",  "BANG",  "BANK",  "BARB",  "BARD",  "BARE",
    "BARK",  "BARN",  "BARR",  "BASE",  "BASH",  "BASK",  "BASS",  "BATE",
    "BATH",  "BAWD",  "BAWL",  "BEAD",  "BEAK",  "BEAM",  "BEAN",  "BEAR",
    "BEAT",  "BEAU",  "BECK",  "BEEF",  "BEEN",  "BEER",  "BEET",  "BELA",
    "BELL",  "BELT",  "BEND",  "BENT",  "BERG",  "BERN",  "BERT",  "BESS",
    "BEST",  "BETA",  "BETH",  "BHOY",  "BIAS",  "BIDE",  "BIEN",  "BILE",
    "BILK",  "BILL",  "BIND",  "BING",  "BIRD",  "BITE",  "BITS",  "BLAB",
    "BLAT",  "BLED",  "BLEW",  "BLOB",  "BLOC",  "BLOT",  "BLOW",  "BLUE",
    "BLUM",  "BLUR",  "BOAR",  "BOAT",  "BOCA",  "BOCK",  "BODE",  "BODY",
    "BOGY",  "BOHR",  "BOIL",  "BOLD",  "BOLO",  "BOLT",  "BOMB",  "BONA",
    "BOND",  "BONE",  "BONG",  "BONN",  "BONY",  "BOOK",  "BOOM",  "BOON",
    "BOOT",  "BORE",  "BORG",  "BORN",  "BOSE",  "BOSS",  "BOTH",  "BOUT",
    "BOWL",  "BOYD",  "BRAD",  "BRAE",  "BRAG",  "BRAN",  "BRAY",  "BRED",
    "BREW",  "BRIG",  "BRIM",  "BROW",  "BUCK",  "BUDD",  "BUFF",  "BULB",
    "BULK",  "BULL",  "BUNK",  "BUNT",  "BUOY",  "BURG",  "BURL",  "BURN",
    "BURR",  "BURT",  "BURY",  "BUSH",  "BUSS",  "BUST",  "BUSY",  "BYTE",
    "CADY",  "CAFE",  "CAGE",  "CAIN",  "CAKE",  "CALF",  "CALL",  "CALM",
    "CAME",  "CANE",  "CANT",  "CARD",  "CARE",  "CARL",  "CARR",  "CART",
    "CASE",  "CASH",  "CASK",  "CAST",  "CAVE",  "CEIL",  "CELL",  "CENT",
    "CERN",  "CHAD",  "CHAR",  "CHAT",  "CHAW",  "CHEF",  "CHEN",  "CHEW",
    "CHIC",  "CHIN",  "CHOU",  "CHOW",  "CHUB",  "CHUG",  "CHUM",  "CITE",
    "CITY",  "CLAD",  "CLAM",  "CLAN",  "CLAW",  "CLAY",  "CLOD",  "CLOG",
    "CLOT",  "CLUB",  "CLUE",  "COAL",  "COAT",  "COCA",  "COCK",  "COCO",
    "CODA",  "CODE",  "CODY",  "COED",  "COIL",  "COIN",  "COKE",  "COLA",
    "COLD",  "COLT",  "COMA",  "COMB",  "COME",  "COOK",  "COOL",  "COON",
    "COOT",  "CORD",  "CORE",  "CORK",  "CORN",  "COST",  "COVE",  "COWL",
    "CRAB",  "CRAG",  "CRAM",  "CRAY",  "CREW",  "CRIB",  "CROW",  "CRUD",
    "CUBA",  "CUBE",  "CUFF",  "CULL",  "CULT",  "CUNY",  "CURB",  "CURD",
    "CURE",  "CURL",  "CURT",  "CUTS",  "DADE",  "DALE",  "DAME",  "DANA",
    "DANE",  "DANG",  "DANK",  "DARE",  "DARK",  "DARN",  "DART",  "DASH",
    "DATA",  "DATE",  "DAVE",  "DAVY",  "DAWN",  "DAYS",  "DEAD",  "DEAF",
    "DEAL",  "DEAN",  "DEAR",  "DEBT",  "DECK",  "DEED",  "DEEM",  "DEER",
    "DEFT",  "DEFY",  "DELL",  "DENT",  "DENY",  "DESK",  "DIAL",  "DICE",
    "DIED",  "DIET",  "DIME",  "DINE",  "DING",  "DINT",  "DIRE",  "DIRT",
    "DISC",  "DISH",  "DISK",  "DIVE",  "DOCK",  "DOES",  "DOLE",  "DOLL",
    "DOLT",  "DOME",  "DONE",  "DOOM",  "DOOR",  "DORA",  "DOSE",  "DOTE",
    "DOUG",  "DOUR",  "DOVE",  "DOWN",  "DRAB",  "DRAG",  "DRAM",  "DRAW",
    "DREW",  "DRUB",  "DRUG",  "DRUM",  "DUAL",  "DUCK",  "DUCT",  "DUEL",
    "DUET",  "DUKE",  "DULL",  "DUMB",  "DUNE",  "DUNK",  "DUSK",  "DUST",
    "DUTY",  "EACH",  "EARL",  "EARN",  "EASE",  "EAST",  "EASY",  "EBEN",
    "ECHO",  "EDDY",  "EDEN",  "EDGE",  "EDGY",  "EDIT",  "EDNA",  "EGAN",
    "ELAN",  "ELBA",  "ELLA",  "ELSE",  "EMIL",  "EMIT",  "EMMA",  "ENDS",
    "ERIC",  "EROS",  "EVEN",  "EVER",  "EVIL",  "EYED",  "FACE",  "FACT",
    "FADE",  "FAIL",  "FAIN",  "FAIR",  "FAKE",  "FALL",  "FAME",  "FANG",
    "FARM",  "FAST",  "FATE",  "FAWN",  "FEAR",  "FEAT",  "FEED",  "FEEL",
    "FEET",  "FELL",  "FELT",  "FEND",  "FERN",  "FEST",  "FEUD",  "FIEF",
    "FIGS",  "FILE",  "FILL",  "FILM",  "FIND",  "FINE",  "FINK",  "FIRE",
    "FIRM",  "FISH",  "FISK",  "FIST",  "FITS",  "FIVE",  "FLAG",  "FLAK",
    "FLAM",  "FLAT",  "FLAW",  "FLEA",  "FLED",  "FLEW",  "FLIT",  "FLOC",
    "FLOG",  "FLOW",  "FLUB",  "FLUE",  "FOAL",  "FOAM",  "FOGY",  "FOIL",
    "FOLD",  "FOLK",  "FOND",  "FONT",  "FOOD",  "FOOL",  "FOOT",  "FORD",
    "FORE",  "FORK",  "FORM",  "FORT",  "FOSS",  "FOUL",  "FOUR",  "FOWL",
    "FRAU",  "FRAY",  "FRED",  "FREE",  "FRET",  "FREY",  "FROG",  "FROM",
    "FUEL",  "FULL",  "FUME",  "FUND",  "FUNK",  "FURY",  "FUSE",  "FUSS",
    "GAFF",  "GAGE",  "GAIL",  "GAIN",  "GAIT",  "GALA",  "GALE",  "GALL",
    "GALT",  "GAME",  "GANG",  "GARB",  "GARY",  "GASH",  "GATE",  "GAUL",
    "GAUR",  "GAVE",  "GAWK",  "GEAR",  "GELD",  "GENE",  "GENT",  "GERM",
    "GETS",  "GIBE",  "GIFT",  "GILD",  "GILL",  "GILT",  "GINA",  "GIRD",
    "GIRL",  "GIST",  "GIVE",  "GLAD",  "GLEE",  "GLEN",  "GLIB",  "GLOB",
    "GLOM",  "GLOW",  "GLUE",  "GLUM",  "GLUT",  "GOAD",  "GOAL",  "GOAT",
    "GOER",  "GOES",  "GOLD",  "GOLF",  "GONE",  "GONG",  "GOOD",  "GOOF",
    "GORE",  "GORY",  "GOSH",  "GOUT",  "GOWN",  "GRAB",  "GRAD",  "GRAY",
    "GREG",  "GREW",  "GREY",  "GRID",  "GRIM",  "GRIN",  "GRIT",  "GROW",
    "GRUB",  "GULF",  "GULL",  "GUNK",  "GURU",  "GUSH",  "GUST",  "GWEN",
    "GWYN",  "HAAG",  "HAAS",  "HACK",  "HAIL",  "HAIR",  "HALE",  "HALF",
    "HALL",  "HALO",  "HALT",  "HAND",  "HANG",  "HANK",  "HANS",  "HARD",
    "HARK",  "HARM",  "HART",  "HASH",  "HAST",  "HATE",  "HATH",  "HAUL",
    "HAVE",  "HAWK",  "HAYS",  "HEAD",  "HEAL",  "HEAR",  "HEAT",  "HEBE",
    "HECK",  "HEED",  "HEEL",  "HEFT",  "HELD",  "HELL",  "HELM",  "HERB",
    "HERD",  "HERE",  "HERO",  "HERS",  "HESS",  "HEWN",  "HICK",  "HIDE",
    "HIGH",  "HIKE",  "HILL",  "HILT",  "HIND",  "HINT",  "HIRE",  "HISS",
    "HIVE",  "HOBO",  "HOCK",  "HOFF",  "HOLD",  "HOLE",  "HOLM",  "HOLT",
    "HOME",  "HONE",  "HONK",  "HOOD",  "HOOF",  "HOOK",  "HOOT",  "HORN",
    "HOSE",  "HOST",  "HOUR",  "HOVE",  "HOWE",  "HOWL",  "HOYT",  "HUCK",
    "HUED",  "HUFF",  "HUGE",  "HUGH",  "HUGO",  "HULK",  "HULL",  "HUNK",
    "HUNT",  "HURD",  "HURL",  "HURT",  "HUSH",  "HYDE",  "HYMN",  "IBIS",
    "ICON",  "IDEA",  "IDLE",  "IFFY",  "INCA",  "INCH",  "INTO",  "IONS",
    "IOTA",  "IOWA",  "IRIS",  "IRMA",  "IRON",  "ISLE",  "ITCH",  "ITEM",
    "IVAN",  "JACK",  "JADE",  "JAIL",  "JAKE",  "JANE",  "JAVA",  "JEAN",
    "JEFF",  "JERK",  "JESS",  "JEST",  "JIBE",  "JILL",  "JILT",  "JIVE",
    "JOAN",  "JOBS",  "JOCK",  "JOEL",  "JOEY",  "JOHN",  "JOIN",  "JOKE",
    "JOLT",  "JOVE",  "JUDD",  "JUDE",  "JUDO",  "JUDY",  "JUJU",  "JUKE",
    "JULY",  "JUNE",  "JUNK",  "JUNO",  "JURY",  "JUST",  "JUTE",  "KAHN",
    "KALE",  "KANE",  "KANT",  "KARL",  "KATE",  "KEEL",  "KEEN",  "KENO",
    "KENT",  "KERN",  "KERR",  "KEYS",  "KICK",  "KILL",  "KIND",  "KING",
    "KIRK",  "KISS",  "KITE",  "KLAN",  "KNEE",  "KNEW",  "KNIT",  "KNOB",
    "KNOT",  "KNOW",  "KOCH",  "KONG",  "KUDO",  "KURD",  "KURT",  "KYLE",
    "LACE",  "LACK",  "LACY",  "LADY",  "LAID",  "LAIN",  "LAIR",  "LAKE",
    "LAMB",  "LAME",  "LAND",  "LANE",  "LANG",  "LARD",  "LARK",  "LASS",
    "LAST",  "LATE",  "LAUD",  "LAVA",  "LAWN",  "LAWS",  "LAYS",  "LEAD",
    "LEAF",  "LEAK",  "LEAN",  "LEAR",  "LEEK",  "LEER",  "LEFT",  "LEND",
    "LENS",  "LENT",  "LEON",  "LESK",  "LESS",  "LEST",  "LETS",  "LIAR",
    "LICE",  "LICK",  "LIED",  "LIEN",  "LIES",  "LIEU",  "LIFE",  "LIFT",
    "LIKE",  "LILA",  "LILT",  "LILY",  "LIMA",  "LIMB",  "LIME",  "LIND",
    "LINE",  "LINK",  "LINT",  "LION",  "LISA",  "LIST",  "LIVE",  "LOAD",
    "LOAF",  "LOAM",  "LOAN",  "LOCK",  "LOFT",  "LOGE",  "LOIS",  "LOLA",
    "LONE",  "LONG",  "LOOK",  "LOON",  "LOOT",  "LORD",  "LORE",  "LOSE",
    "LOSS",  "LOST",  "LOUD",  "LOVE",  "LOWE",  "LUCK",  "LUCY",  "LUGE",
    "LUKE",  "LULU",  "LUND",  "LUNG",  "LURA",  "LURE",  "LURK",  "LUSH",
    "LUST",  "LYLE",  "LYNN",  "LYON",  "LYRA",  "MACE",  "MADE",  "MAGI",
    "MAID",  "MAIL",  "MAIN",  "MAKE",  "MALE",  "MALI",  "MALL",  "MALT",
    "MANA",  "MANN",  "MANY",  "MARC",  "MARE",  "MARK",  "MARS",  "MART",
    "MARY",  "MASH",  "MASK",  "MASS",  "MAST",  "MATE",  "MATH",  "MAUL",
    "MAYO",  "MEAD",  "MEAL",  "MEAN",  "MEAT",  "MEEK",  "MEET",  "MELD",
    "MELT",  "MEMO",  "MEND",  "MENU",  "MERT",  "MESH",  "MESS",  "MICE",
    "MIKE",  "MILD",  "MILE",  "MILK",  "MILL",  "MILT",  "MIMI",  "MIND",
    "MINE",  "MINI",  "MINK",  "MINT",  "MIRE",  "MISS",  "MIST",  "MITE",
    "MITT",  "MOAN",  "MOAT",  "MOCK",  "MODE",  "MOLD",  "MOLE",  "MOLL",
    "MOLT",  "MONA",  "MONK",  "MONT",  "MOOD",  "MOON",  "MOOR",  "MOOT",
    "MORE",  "MORN",  "MORT",  "MOSS",  "MOST",  "MOTH",  "MOVE",  "MUCH",
    "MUCK",  "MUDD",  "MUFF",  "MULE",  "MULL",  "MURK",  "MUSH",  "MUST",
    "MUTE",  "MUTT",  "MYRA",  "MYTH",  "NAGY",  "NAIL",  "NAIR",  "NAME",
    "NARY",  "NASH",  "NAVE",  "NAVY",  "NEAL",  "NEAR",  "NEAT",  "NECK",
    "NEED",  "NEIL",  "NELL",  "NEON",  "NERO",  "NESS",  "NEST",  "NEWS",
    "NEWT",  "NIBS",  "NICE",  "NICK",  "NILE",  "NINA",  "NINE",  "NOAH",
    "NODE",  "NOEL",  "NOLL",  "NONE",  "NOOK",  "NOON",  "NORM",  "NOSE",
    "NOTE",  "NOUN",  "NOVA",  "NUDE",  "NULL",  "NUMB",  "OATH",  "OBEY",
    "OBOE",  "ODIN",  "OHIO",  "OILY",  "OINT",  "OKAY",  "OLAF",  "OLDY",
    "OLGA",  "OLIN",  "OMAN",  "OMEN",  "OMIT",  "ONCE",  "ONES",  "ONLY",
    "ONTO",  "ONUS",  "ORAL",  "ORGY",  "OSLO",  "OTIS",  "OTTO",  "OUCH",
    "OUST",  "OUTS",  "OVAL",  "OVEN",  "OVER",  "OWLY",  "OWNS",  "QUAD",
    "QUIT",  "QUOD",  "RACE",  "RACK",  "RACY",  "RAFT",  "RAGE",  "RAID",
    "RAIL",  "RAIN",  "RAKE",  "RANK",  "RANT",  "RARE",  "RASH",  "RATE",
    "RAVE",  "RAYS",  "READ",  "REAL",  "REAM",  "REAR",  "RECK",  "REED",
    "REEF",  "REEK",  "REEL",  "REID",  "REIN",  "RENA",  "REND",  "RENT",
    "REST",  "RICE",  "RICH",  "RICK",  "RIDE",  "RIFT",  "RILL",  "RIME",
    "RING",  "RINK",  "RISE",  "RISK",  "RITE",  "ROAD",  "ROAM",  "ROAR",
    "ROBE",  "ROCK",  "RODE",  "ROIL",  "ROLL",  "ROME",  "ROOD",  "ROOF",
    "ROOK",  "ROOM",  "ROOT",  "ROSA",  "ROSE",  "ROSS",  "ROSY",  "ROTH",
    "ROUT",  "ROVE",  "ROWE",  "ROWS",  "RUBE",  "RUBY",  "RUDE",  "RUDY",
    "RUIN",  "RULE",  "RUNG",  "RUNS",  "RUNT",  "RUSE",  "RUSH",  "RUSK",
    "RUSS",  "RUST",  "RUTH",  "SACK",  "SAFE",  "SAGE",  "SAID",  "SAIL",
    "SALE",  "SALK",  "SALT",  "SAME",  "SAND",  "SANE",  "SANG",  "SANK",
    "SARA",  "SAUL",  "SAVE",  "SAYS",  "SCAN",  "SCAR",  "SCAT",  "SCOT",
    "SEAL",  "SEAM",  "SEAR",  "SEAT",  "SEED",  "SEEK",  "SEEM",  "SEEN",
    "SEES",  "SELF",  "SELL",  "SEND",  "SENT",  "SETS",  "SEWN",  "SHAG",
    "SHAM",  "SHAW",  "SHAY",  "SHED",  "SHIM",  "SHIN",  "SHOD",  "SHOE",
    "SHOT",  "SHOW",  "SHUN",  "SHUT",  "SICK",  "SIDE",  "SIFT",  "SIGH",
    "SIGN",  "SILK",  "SILL",  "SILO",  "SILT",  "SINE",  "SING",  "SINK",
    "SIRE",  "SITE",  "SITS",  "SITU",  "SKAT",  "SKEW",  "SKID",  "SKIM",
    "SKIN",  "SKIT",  "SLAB",  "SLAM",  "SLAT",  "SLAY",  "SLED",  "SLEW",
    "SLID",  "SLIM",  "SLIT",  "SLOB",  "SLOG",  "SLOT",  "SLOW",  "SLUG",
    "SLUM",  "SLUR",  "SMOG",  "SMUG",  "SNAG",  "SNOB",  "SNOW",  "SNUB",
    "SNUG",  "SOAK",  "SOAR",  "SOCK",  "SODA",  "SOFA",  "SOFT",  "SOIL",
    "SOLD",  "SOME",  "SONG",  "SOON",  "SOOT",  "SORE",  "SORT",  "SOUL",
    "SOUR",  "SOWN",  "STAB",  "STAG",  "STAN",  "STAR",  "STAY",  "STEM",
    "STEW",  "STIR",  "STOW",  "STUB",  "STUN",  "SUCH",  "SUDS",  "SUIT",
    "SULK",  "SUMS",  "SUNG",  "SUNK",  "SURE",  "SURF",  "SWAB",  "SWAG",
    "SWAM",  "SWAN",  "SWAT",  "SWAY",  "SWIM",  "SWUM",  "TACK",  "TACT",
    "TAIL",  "TAKE",  "TALE",  "TALK",  "TALL",  "TANK",  "TASK",  "TATE",
    "TAUT",  "TEAL",  "TEAM",  "TEAR",  "TECH",  "TEEM",  "TEEN",  "TEET",
    "TELL",  "TEND",  "TENT",  "TERM",  "TERN",  "TESS",  "TEST",  "THAN",
    "THAT",  "THEE",  "THEM",  "THEN",  "THEY",  "THIN",  "THIS",  "THUD",
    "THUG",  "TICK",  "TIDE",  "TIDY",  "TIED",  "TIER",  "TILE",  "TILL",
    "TILT",  "TIME",  "TINA",  "TINE",  "TINT",  "TINY",  "TIRE",  "TOAD",
    "TOGO",  "TOIL",  "TOLD",  "TOLL",  "TONE",  "TONG",  "TONY",  "TOOK",
    "TOOL",  "TOOT",  "TORE",  "TORN",  "TOTE",  "TOUR",  "TOUT",  "TOWN",
    "TRAG",  "TRAM",  "TRAY",  "TREE",  "TREK",  "TRIG",  "TRIM",  "TRIO",
    "TROD",  "TROT",  "TROY",  "TRUE",  "TUBA",  "TUBE",  "TUCK",  "TUFT",
    "TUNA",  "TUNE",  "TUNG",  "TURF",  "TURN",  "TUSK",  "TWIG",  "TWIN",
    "TWIT",  "ULAN",  "UNIT",  "URGE",  "USED",  "USER",  "USES",  "UTAH",
    "VAIL",  "VAIN",  "VALE",  "VARY",  "VASE",  "VAST",  "VEAL",  "VEDA",
    "VEIL",  "VEIN",  "VEND",  "VENT",  "VERB",  "VERY",  "VETO",  "VICE",
    "VIEW",  "VINE",  "VISE",  "VOID",  "VOLT",  "VOTE",  "WACK",  "WADE",
    "WAGE",  "WAIL",  "WAIT",  "WAKE",  "WALE",  "WALK",  "WALL",  "WALT",
    "WAND",  "WANE",  "WANG",  "WANT",  "WARD",  "WARM",  "WARN",  "WART",
    "WASH",  "WAST",  "WATS",  "WATT",  "WAVE",  "WAVY",  "WAYS",  "WEAK",
    "WEAL",  "WEAN",  "WEAR",  "WEED",  "WEEK",  "WEIR",  "WELD",  "WELL",
    "WELT",  "WENT",  "WERE",  "WERT",  "WEST",  "WHAM",  "WHAT",  "WHEE",
    "WHEN",  "WHET",  "WHOA",  "WHOM",  "WICK",  "WIFE",  "WILD",  "WILL",
    "WIND",  "WINE",  "WING",  "WINK",  "WINO",  "WIRE",  "WISE",  "WISH",
    "WITH",  "WOLF",  "WONT",  "WOOD",  "WOOL",  "WORD",  "WORE",  "WORK",
    "WORM",  "WORN",  "WOVE",  "WRIT",  "WYNN",  "YALE",  "YANG",  "YANK",
    "YARD",  "YARN",  "YAWL",  "YAWN",  "YEAH",  "YEAR",  "YELL",  "YOGA",
    "YOKE"
];

#[cfg(test)]
mod tests {
    use crate::hash::sixwords::{
        fold_hash_to_u64, get_six_words_by_value, get_six_words_for_md5, get_six_words_for_sha1,
        get_value_for_six_words,
    };
    use crate::hash::{MD5, SHA1};

    #[test]
    pub fn test1() {
        let words = get_six_words_by_value(0xD1854218EBBB0B51u64);
        let exp = ["ROME", "MUG", "FRED", "SCAN", "LIVE", "LACE"];
        assert_eq!(exp, words);
        assert_eq!(Ok(0xD1854218EBBB0B51u64), get_value_for_six_words(&exp));
        let exp = ["rome", "MuG", "Fred", "sCaN", "LIVE", "LaCe"];
        assert_eq!(Ok(0xD1854218EBBB0B51u64), get_value_for_six_words(&exp));

        let words = get_six_words_by_value(0xBB9E6AE1979D8FF4u64);
        assert_eq!(["MILT", "VARY", "MAST", "OK", "SEES", "WENT"], words);
    }

    #[test]
    pub fn test_sha1() {
        let hash = SHA1::new().hash(b"This is a test.TeSt");
        assert_eq_hex_slice!(
            &[
                0x8d, 0xa9, 0xcd, 0x1a, 0x74, 0xb5, 0x81, 0x4f, 0x11, 0x71, 0xfa, 0x6e, 0x0f, 0x51,
                0x3b, 0x15, 0x01, 0x8c, 0x9b, 0x02
            ],
            &hash
        );
        let value = fold_hash_to_u64(&hash);
        assert_eq_hex!(0x5ABAE47B76AC549D, value);
        let words = get_six_words_by_value(value);
        assert_eq!(["BODE", "SAVE", "IT", "TOLL", "NONE", "ASKS"], words);

        let words = get_six_words_for_sha1(b"This is a test.TeSt");
        assert_eq!(["BODE", "SAVE", "IT", "TOLL", "NONE", "ASKS"], words);
    }

    #[test]
    pub fn test_md5() {
        let hash = MD5::default().hash(b"This is a test.TeSt").to_be_bytes();
        assert_eq_hex_slice!(
            &[
                0xe2, 0xfd, 0x45, 0x5f, 0xf5, 0x55, 0x57, 0xb7, 0x1a, 0xec, 0xd9, 0x26, 0x0f, 0xde,
                0xe6, 0x2b
            ],
            &hash
        );
        let value = fold_hash_to_u64(&hash);
        assert_eq_hex!(0x9CB18BFA799C11F8, value);
        let words = get_six_words_by_value(value);
        assert_eq!(["HULL", "GLAD", "YALE", "VALE", "MUTT", "WINE"], words);

        let words = get_six_words_for_md5(b"This is a test.TeSt");
        assert_eq!(["HULL", "GLAD", "YALE", "VALE", "MUTT", "WINE"], words);
    }
}

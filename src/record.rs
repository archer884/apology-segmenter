use std::collections::HashMap;
use rustc_serialize::{Decodable, Decoder};

lazy_static! {
    static ref TRANSLATOR: HashMap<String, String> = create_translator();
}

#[derive(Debug, RustcEncodable)]
pub struct Record {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
    pub country: String,
    pub phone: String,
}

impl Record {
    pub fn key(&self) -> String {
        format!("{}_{}", self.country, self.region)
    }
}

impl Decodable for Record {
    fn decode<T: Decoder>(decoder: &mut T) -> Result<Record, T::Error> {
        Ok(Record {
            id: try!(decoder.read_str()).trim().to_owned(),
            email: try!(decoder.read_str()).trim().to_owned(),
            first_name: try!(decoder.read_str()).trim().to_owned(),
            last_name: try!(decoder.read_str()).trim().to_owned(),
            address1: try!(decoder.read_str()).trim().to_owned(),
            address2: String::new(),
            city: try!(decoder.read_str()).trim().to_owned(),
            region: try!(decoder.read_str()).trim().to_owned(),
            postal_code: try!(decoder.read_str()).trim().to_owned(),
            country: translate(&format!("{:0>4}", try!(decoder.read_str()))),
            phone: clean_phone_number(try!(decoder.read_str())),
        })
    }
}

fn translate(code: &str) -> String {
    TRANSLATOR.get(code).map_or_else(|| code.to_owned(), |entry| entry.to_owned())
}

fn clean_phone_number<T>(phone: T) -> String 
    where T: AsRef<str> + Into<String>
{
    if phone.as_ref().contains("+") {
        String::new()
    } else {
        phone.into()
    }
}

fn create_translator() -> HashMap<String, String> {
    let mut map = HashMap::new();
    
    map.insert("1000".to_owned(), "AFG".to_owned());
    map.insert("1001".to_owned(), "ALB".to_owned());
    map.insert("1002".to_owned(), "DZA".to_owned());
    map.insert("1003".to_owned(), "AND".to_owned());
    map.insert("1004".to_owned(), "AGO".to_owned());
    map.insert("1213".to_owned(), "AIA".to_owned());
    map.insert("1214".to_owned(), "ATG".to_owned());
    map.insert("1005".to_owned(), "ARG".to_owned());
    map.insert("1215".to_owned(), "ARM".to_owned());
    map.insert("1216".to_owned(), "ABW".to_owned());
    map.insert("1006".to_owned(), "ASI".to_owned());
    map.insert("1007".to_owned(), "AUS".to_owned());
    map.insert("1008".to_owned(), "AUT".to_owned());
    map.insert("1217".to_owned(), "AZE".to_owned());
    map.insert("1009".to_owned(), "AZR".to_owned());
    map.insert("1010".to_owned(), "BHS".to_owned());
    map.insert("1011".to_owned(), "BHR".to_owned());
    map.insert("1012".to_owned(), "BGD".to_owned());
    map.insert("1013".to_owned(), "BRB".to_owned());
    map.insert("1218".to_owned(), "BLR".to_owned());
    map.insert("1014".to_owned(), "BEL".to_owned());
    map.insert("1015".to_owned(), "BLZ".to_owned());
    map.insert("1016".to_owned(), "BEN".to_owned());
    map.insert("1017".to_owned(), "BMU".to_owned());
    map.insert("1018".to_owned(), "BTN".to_owned());
    map.insert("1019".to_owned(), "BOL".to_owned());
    map.insert("1206".to_owned(), "BIH".to_owned());
    map.insert("1021".to_owned(), "BWA".to_owned());
    map.insert("1022".to_owned(), "BRA".to_owned());
    map.insert("1219".to_owned(), "VGB".to_owned());
    map.insert("1024".to_owned(), "BRN".to_owned());
    map.insert("1025".to_owned(), "BGR".to_owned());
    map.insert("1195".to_owned(), "BFA".to_owned());
    map.insert("1026".to_owned(), "MMR".to_owned());
    map.insert("1027".to_owned(), "BDI".to_owned());
    map.insert("1028".to_owned(), "KHM".to_owned());
    map.insert("1029".to_owned(), "CMR".to_owned());
    map.insert("0001".to_owned(), "CAN".to_owned());
    map.insert("1030".to_owned(), "CNI".to_owned());
    map.insert("1032".to_owned(), "CPV".to_owned());
    map.insert("1034".to_owned(), "CYM".to_owned());
    map.insert("1035".to_owned(), "CAF".to_owned());
    map.insert("1036".to_owned(), "TCD".to_owned());
    map.insert("1037".to_owned(), "CHI".to_owned());
    map.insert("1038".to_owned(), "CHL".to_owned());
    map.insert("1039".to_owned(), "CHN".to_owned());
    map.insert("1040".to_owned(), "COL".to_owned());
    map.insert("1041".to_owned(), "COM".to_owned());
    map.insert("1042".to_owned(), "COG".to_owned());
    map.insert("1222".to_owned(), "COK".to_owned());
    map.insert("1044".to_owned(), "CRI".to_owned());
    map.insert("1223".to_owned(), "HRV".to_owned());
    map.insert("1046".to_owned(), "CYP".to_owned());
    map.insert("1047".to_owned(), "CZE".to_owned());
    map.insert("1048".to_owned(), "DNK".to_owned());
    map.insert("1049".to_owned(), "DJI".to_owned());
    map.insert("1050".to_owned(), "DMA".to_owned());
    map.insert("1051".to_owned(), "DOM".to_owned());
    map.insert("1052".to_owned(), "TLS".to_owned());
    map.insert("1053".to_owned(), "ECU".to_owned());
    map.insert("1054".to_owned(), "EGY".to_owned());
    map.insert("1055".to_owned(), "SLV".to_owned());
    map.insert("1056".to_owned(), "GNQ".to_owned());
    map.insert("1057".to_owned(), "EST".to_owned());
    map.insert("1058".to_owned(), "ETH".to_owned());
    map.insert("1059".to_owned(), "FRO".to_owned());
    map.insert("1061".to_owned(), "FJI".to_owned());
    map.insert("1062".to_owned(), "FIN".to_owned());
    map.insert("1063".to_owned(), "FRA".to_owned());
    map.insert("1064".to_owned(), "GUF".to_owned());
    map.insert("1065".to_owned(), "PYF".to_owned());
    map.insert("1066".to_owned(), "GAB".to_owned());
    map.insert("1067".to_owned(), "GMB".to_owned());
    map.insert("1225".to_owned(), "GEO".to_owned());
    map.insert("1068".to_owned(), "DEU".to_owned());
    map.insert("1070".to_owned(), "GHA".to_owned());
    map.insert("1071".to_owned(), "GIB".to_owned());
    map.insert("1073".to_owned(), "GRC".to_owned());
    map.insert("1074".to_owned(), "GRL".to_owned());
    map.insert("1075".to_owned(), "GRD".to_owned());
    map.insert("1076".to_owned(), "GLP".to_owned());
    map.insert("1078".to_owned(), "GTM".to_owned());
    map.insert("1079".to_owned(), "GIN".to_owned());
    map.insert("1080".to_owned(), "GNB".to_owned());
    map.insert("1081".to_owned(), "GUY".to_owned());
    map.insert("1082".to_owned(), "HTI".to_owned());
    map.insert("1083".to_owned(), "HND".to_owned());
    map.insert("1084".to_owned(), "HKG".to_owned());
    map.insert("1085".to_owned(), "HUN".to_owned());
    map.insert("1086".to_owned(), "ISL".to_owned());
    map.insert("1087".to_owned(), "IND".to_owned());
    map.insert("1088".to_owned(), "IDN".to_owned());
    map.insert("1091".to_owned(), "IRL".to_owned());
    map.insert("1251".to_owned(), "IMN".to_owned());
    map.insert("1092".to_owned(), "ISR".to_owned());
    map.insert("1093".to_owned(), "ITA".to_owned());
    map.insert("1094".to_owned(), "CIV".to_owned());
    map.insert("1095".to_owned(), "JAM".to_owned());
    map.insert("1096".to_owned(), "JPN".to_owned());
    map.insert("1097".to_owned(), "JOR".to_owned());
    map.insert("1226".to_owned(), "KAZ".to_owned());
    map.insert("1099".to_owned(), "KEN".to_owned());
    map.insert("1100".to_owned(), "KIR".to_owned());
    map.insert("1101".to_owned(), "KOR".to_owned());
    
    // There appears to be some kind of bug where this guy has Seoul as his city but 
    // then has a country code that matches exactly NOTHING, so...
    map.insert("1152".to_owned(), "KOR".to_owned());
    
    map.insert("1102".to_owned(), "KWT".to_owned());
    map.insert("1227".to_owned(), "KGZ".to_owned());
    map.insert("1103".to_owned(), "LAO".to_owned());
    map.insert("1104".to_owned(), "LVA".to_owned());
    map.insert("1105".to_owned(), "LBN".to_owned());
    map.insert("1107".to_owned(), "LSO".to_owned());
    map.insert("1108".to_owned(), "LBR".to_owned());
    map.insert("1109".to_owned(), "LBY".to_owned());
    map.insert("1110".to_owned(), "LIE".to_owned());
    map.insert("1111".to_owned(), "LTU".to_owned());
    map.insert("1112".to_owned(), "LUX".to_owned());
    map.insert("1113".to_owned(), "MAC".to_owned());
    map.insert("1228".to_owned(), "MKD".to_owned());
    map.insert("1114".to_owned(), "MDG".to_owned());
    map.insert("1115".to_owned(), "MAD".to_owned());
    map.insert("1116".to_owned(), "MWI".to_owned());
    map.insert("1117".to_owned(), "MYS".to_owned());
    map.insert("1118".to_owned(), "MDV".to_owned());
    map.insert("1119".to_owned(), "MLI".to_owned());
    map.insert("1120".to_owned(), "MLT".to_owned());
    map.insert("1121".to_owned(), "MTQ".to_owned());
    map.insert("1123".to_owned(), "MUS".to_owned());
    map.insert("1122".to_owned(), "MRT".to_owned());
    map.insert("1124".to_owned(), "MEX".to_owned());
    map.insert("1231".to_owned(), "MDA".to_owned());
    map.insert("1125".to_owned(), "MCO".to_owned());
    map.insert("1126".to_owned(), "MNG".to_owned());
    map.insert("1232".to_owned(), "MNE".to_owned());
    map.insert("1233".to_owned(), "MSR".to_owned());
    map.insert("1127".to_owned(), "MAR".to_owned());
    map.insert("1128".to_owned(), "MOZ".to_owned());
    map.insert("1130".to_owned(), "NAU".to_owned());
    map.insert("1131".to_owned(), "NPL".to_owned());
    map.insert("1132".to_owned(), "NLD".to_owned());
    map.insert("1133".to_owned(), "ANT".to_owned());
    map.insert("1134".to_owned(), "NCL".to_owned());
    map.insert("1135".to_owned(), "NZL".to_owned());
    map.insert("1136".to_owned(), "NIC".to_owned());
    map.insert("1137".to_owned(), "NER".to_owned());
    map.insert("1138".to_owned(), "NGA".to_owned());
    map.insert("1139".to_owned(), "NID".to_owned());
    map.insert("1140".to_owned(), "NOR".to_owned());
    map.insert("1141".to_owned(), "OMN".to_owned());
    map.insert("1142".to_owned(), "PAK".to_owned());
    map.insert("1235".to_owned(), "PLW".to_owned());
    map.insert("1143".to_owned(), "PAN".to_owned());
    map.insert("1144".to_owned(), "PNG".to_owned());
    map.insert("1145".to_owned(), "PRY".to_owned());
    map.insert("1146".to_owned(), "PER".to_owned());
    map.insert("1147".to_owned(), "PHL".to_owned());
    map.insert("1148".to_owned(), "PIT".to_owned());
    map.insert("1149".to_owned(), "POL".to_owned());
    map.insert("1150".to_owned(), "PRT".to_owned());
    map.insert("1151".to_owned(), "QAT".to_owned());
    map.insert("1153".to_owned(), "REU".to_owned());
    map.insert("1155".to_owned(), "ROU".to_owned());
    map.insert("1193".to_owned(), "RUS".to_owned());
    map.insert("1156".to_owned(), "RWA".to_owned());
    map.insert("1129".to_owned(), "NAM".to_owned());
    map.insert("1238".to_owned(), "SAB".to_owned());
    map.insert("1157".to_owned(), "SHN".to_owned());
    map.insert("1158".to_owned(), "LCA".to_owned());
    map.insert("1241".to_owned(), "SMR".to_owned());
    map.insert("1161".to_owned(), "STP".to_owned());
    map.insert("1162".to_owned(), "SAU".to_owned());
    map.insert("1163".to_owned(), "SCT".to_owned());
    map.insert("1164".to_owned(), "SEN".to_owned());
    map.insert("1243".to_owned(), "SRB".to_owned());
    map.insert("1165".to_owned(), "SYC".to_owned());
    map.insert("1166".to_owned(), "SLE".to_owned());
    map.insert("1167".to_owned(), "SGP".to_owned());
    map.insert("1245".to_owned(), "SVK".to_owned());
    map.insert("1210".to_owned(), "SVN".to_owned());
    map.insert("1168".to_owned(), "SLB".to_owned());
    map.insert("1171".to_owned(), "ZAF".to_owned());
    map.insert("1172".to_owned(), "ESP".to_owned());
    map.insert("1173".to_owned(), "LKA".to_owned());
    map.insert("1174".to_owned(), "VCT".to_owned());
    map.insert("1240".to_owned(), "KNA".to_owned());
    map.insert("1175".to_owned(), "SDN".to_owned());
    map.insert("1176".to_owned(), "SUR".to_owned());
    map.insert("1177".to_owned(), "SWZ".to_owned());
    map.insert("1178".to_owned(), "SWE".to_owned());
    map.insert("1179".to_owned(), "CHE".to_owned());
    map.insert("1181".to_owned(), "TWN".to_owned());
    map.insert("1246".to_owned(), "TJK".to_owned());
    map.insert("1182".to_owned(), "TZA".to_owned());
    map.insert("1183".to_owned(), "THA".to_owned());
    map.insert("1184".to_owned(), "TGO".to_owned());
    map.insert("1185".to_owned(), "TON".to_owned());
    map.insert("1186".to_owned(), "TTO".to_owned());
    map.insert("1187".to_owned(), "TDC".to_owned());
    map.insert("1188".to_owned(), "TUN".to_owned());
    map.insert("1189".to_owned(), "TUR".to_owned());
    map.insert("1247".to_owned(), "TKM".to_owned());
    map.insert("1190".to_owned(), "TCA".to_owned());
    map.insert("1191".to_owned(), "TUV".to_owned());
    map.insert("1192".to_owned(), "UGA".to_owned());
    map.insert("1248".to_owned(), "UKR".to_owned());
    map.insert("1194".to_owned(), "ARE".to_owned());
    map.insert("1072".to_owned(), "GBR".to_owned());
    map.insert("1196".to_owned(), "URY".to_owned());
    map.insert("0000".to_owned(), "USA".to_owned());
    map.insert("1250".to_owned(), "UZB".to_owned());
    map.insert("1197".to_owned(), "VUT".to_owned());
    map.insert("1198".to_owned(), "VTC".to_owned());
    map.insert("1199".to_owned(), "VEN".to_owned());
    map.insert("1200".to_owned(), "VNM".to_owned());
    map.insert("1202".to_owned(), "WLS".to_owned());
    map.insert("1203".to_owned(), "WSM".to_owned());
    map.insert("1204".to_owned(), "YEM".to_owned());
    map.insert("1207".to_owned(), "ZAI".to_owned());
    map.insert("1208".to_owned(), "ZMB".to_owned());
    map.insert("1209".to_owned(), "ZWE".to_owned());
    
    map
}
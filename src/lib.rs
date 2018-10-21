//! # spanish-numbers
//!
//! A library for converting integers to their written spanish formats.  
//! Supports both American "short" and European "long" number formats.

#[derive(Copy, Clone)]
pub enum ScaleType {
    Short,
    Long
}

struct NamedNumber {
    number: u128,
    singular_name: &'static str,
    plural_name: &'static str
}

impl NamedNumber {

    pub fn new(number: u128, singular_name: &'static str, plural_name: &'static str) -> NamedNumber {
        NamedNumber { number, singular_name, plural_name }
    }
}

pub struct NumberToSpanish {
    scale: Vec<NamedNumber>
}

impl NumberToSpanish {

    pub fn new(scale_type: ScaleType) -> NumberToSpanish {
        let scale = Self::get_scale(scale_type);
        NumberToSpanish { scale }
    }

    fn get_scale(scale_type: ScaleType) -> Vec<NamedNumber> {
        match scale_type {
            ScaleType::Short => vec![
                NamedNumber::new(1000000000000000000000000000000000000, "undecillón", "undecillones"),
                NamedNumber::new(1000000000000000000000000000000000, "decillón", "decillones"),
                NamedNumber::new(1000000000000000000000000000000, "nonillón", "nonillones"),
                NamedNumber::new(1000000000000000000000000000, "octillón", "octillones"),
                NamedNumber::new(1000000000000000000000000, "septillón", "septillones"),
                NamedNumber::new(1000000000000000000000, "sextillón", "sextillones"),
                NamedNumber::new(1000000000000000000, "quintillón", "quintillones"),
                NamedNumber::new(1000000000000000, "cuatrillón", "cuatrillones"),
                NamedNumber::new(1000000000000, "trillón", "trillones"),
                NamedNumber::new(1000000000, "billón", "billones"),
                NamedNumber::new(1000000, "millón", "millones"),
                NamedNumber::new(1, "", "")
            ],
            ScaleType::Long => vec![
                NamedNumber::new(1000000000000000000000000000000000000, "sextillón", "sextillones"),
                NamedNumber::new(1000000000000000000000000000000, "quintillón", "quintillones"),
                NamedNumber::new(1000000000000000000000000, "cuatrillón", "cuatrillones"),
                NamedNumber::new(1000000000000000000, "trillón", "trillones"),
                NamedNumber::new(1000000000000, "billón", "billones"),
                NamedNumber::new(1000000, "millón", "millones"),
                NamedNumber::new(1, "", "")
            ]
        }
    }

    pub fn number_to_spanish(&self, number: u128, separator: &str) -> String {
        match number {
            0 => "cero".to_owned(),
            _ => self.translate_positive_number(number).join(separator)
        }
    }

    fn translate_positive_number(&self, number: u128) -> Vec<String> {
        let mut translations = vec![];
        let mut translation = String::from("");
        
        let (divisor, divisor_name) = self.get_greatest_divisor(number);
        let fst_num = number / divisor;
        let snd_num = number % divisor;
        
        if fst_num > 0 {
            let is_not_end_of_sentence = snd_num > 0 || divisor > 1;
            let fst_num_translation = Self::translate_1_until_million(fst_num, is_not_end_of_sentence);
            translation.push_str(&fst_num_translation);
        }
        if translation.len() > 0 {
            if divisor_name.len() > 0 { 
                translation.push(' ');
                translation.push_str(&divisor_name);
            }
            translations.push(translation);
        }
        if snd_num > 0 {
            translations.extend(self.translate_positive_number(snd_num));
        }
        translations
    }
    
    fn get_greatest_divisor(&self, number: u128) -> (u128, String) {
        let divisor = self.scale.iter().find(|d| d.number <= number).unwrap();
        let v = number / divisor.number;
        let divisor_name = if v > 1 { divisor.plural_name } else { divisor.singular_name };
        (divisor.number, divisor_name.to_owned())
    }

    fn translate_1_until_million(number: u128, is_not_end: bool) -> String {
        match number {
            1...999 => Self::translate_1_to_999(number, is_not_end),
            1000...999999 => Self::translate_1000_until_million(number, is_not_end),
            _ => panic!("number {}", number)
        }
    }

    fn translate_1_to_999(number: u128, is_not_end: bool) -> String {
        match number {
            1...99 => Self::translate_1_to_99(number, is_not_end),
            100 => "cien".to_owned(),
            101...999 => Self::translate_101_to_999(number, is_not_end),
            _ => panic!("number {}", number)
        }
    }

    fn translate_1000_until_million(number: u128, is_not_end: bool) -> String {
        let fst_num = number / 1000;
        let snd_digit = number % 1000;
        let mut translation = String::from("");
        if fst_num > 1 {
            translation.push_str(&Self::translate_1_to_999(fst_num, is_not_end || snd_digit > 0));
            translation.push(' ');
        }
        translation.push_str("mil");
        if snd_digit > 0 {
            translation.push(' ');
            translation.push_str(&Self::translate_1_to_999(snd_digit, is_not_end));
        }
        translation
    }

    fn translate_1_to_99(number: u128, is_not_end: bool) -> String {
        match number {
            1...30 => Self::translate_from_1_to_30(number, is_not_end),
            31...99 => Self::translate_31_to_99(number, is_not_end),
            _ => panic!("number {}", number)
        }
    }

    fn translate_101_to_999(number: u128, is_not_end: bool) -> String {
        let fst_digit = number / 100;
        let snd_digit = number % 100;
        let mut translation = Self::match_multiples_of_100_to_900(fst_digit);
        if snd_digit > 0 {
            translation.push(' ');
            translation.push_str(&Self::translate_1_to_99(snd_digit, is_not_end));
        }
        translation
    }

    fn translate_from_1_to_30(number: u128, is_not_end: bool) -> String {
        (match number {
            n if n == 1 && is_not_end => "un",
            1 => "uno",
            2 => "dos",
            3 => "tres",
            4 => "cuatro",
            5 => "cinco",
            6 => "seis",
            7 => "siete",
            8 => "ocho",
            9 => "nueve",
            10 => "diez",
            11 => "once",
            12 => "doce",
            13 => "trece",
            14 => "catorce",
            15 => "quince",
            16 => "dieciséis",
            17 => "diecisiete",
            18 => "dieciocho",
            19 => "diecinueve",
            20 => "veinte",
            21 => "veintiuno",
            22 => "veintidós",
            23 => "veintitrés",
            24 => "venticuatro",
            25 => "veinticinco",
            26 => "veintiséis",
            27 => "veintisiete",
            28 => "veintiocho",
            29 => "veintinueve",
            30 => "treinta",
            _ => panic!("number {}", number)
        }).to_owned()
    }

    fn translate_31_to_99(number: u128, is_not_end: bool) -> String {
        let fst_digit = number / 10;
        let snd_digit = number % 10;
        let mut translation = Self::match_multiples_of_10_to_90(fst_digit);
        if snd_digit > 0 {
            translation.push_str(" y ");
            translation.push_str(&Self::translate_from_1_to_30(snd_digit, is_not_end));
        }
        translation
    }

    fn match_multiples_of_10_to_90(number: u128) -> String {
        (match number {
            1 => "diez",
            2 => "veinte",
            3 => "treinta",
            4 => "cuarenta",
            5 => "cincuenta",
            6 => "sesenta",
            7 => "setenta",
            8 => "ochenta",
            9 => "noventa",
            _ => panic!("number {}", number)
        }).to_owned()
    }

    fn match_multiples_of_100_to_900(number: u128) -> String {
        (match number {
            1 => "ciento",
            2 => "doscientos",
            3 => "trescientos",
            4 => "cuatrocientos",
            5 => "quinientos",
            6 => "seiscientos",
            7 => "setecientos",
            8 => "ochocientos",
            9 => "novecientos",
            _ => panic!("number {}", number)
        }).to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nsl = NumberToSpanish::new(ScaleType::Long);
        let nss = NumberToSpanish::new(ScaleType::Short);

        assert_eq!(nsl.number_to_spanish(1, " "), "uno");
        assert_eq!(nss.number_to_spanish(1, " "), "uno");
        assert_eq!(nsl.number_to_spanish(1000, " "), "mil");
        assert_eq!(nss.number_to_spanish(1000, " "), "mil");
        assert_eq!(nsl.number_to_spanish(1000000, " "), "un millón");
        assert_eq!(nss.number_to_spanish(1000000, " "), "un millón");
        assert_eq!(nsl.number_to_spanish(1000000000, " "), "mil millones");
        assert_eq!(nss.number_to_spanish(1000000000, " "), "un billón");
        assert_eq!(nsl.number_to_spanish(1000000000000, " "), "un billón");
        assert_eq!(nss.number_to_spanish(1000000000000, " "), "un trillón");
        assert_eq!(nsl.number_to_spanish(1000000000000000, " "), "mil billones");
        assert_eq!(nss.number_to_spanish(1000000000000000, " "), "un cuatrillón");
        assert_eq!(nsl.number_to_spanish(1000000000000000000, " "), "un trillón");
        assert_eq!(nss.number_to_spanish(1000000000000000000, " "), "un quintillón");
        assert_eq!(nsl.number_to_spanish(1000000000000000000000, " "), "mil trillones");
        assert_eq!(nss.number_to_spanish(1000000000000000000000, " "), "un sextillón");
        assert_eq!(nsl.number_to_spanish(1000000000000000000000000, " "), "un cuatrillón");
        assert_eq!(nss.number_to_spanish(1000000000000000000000000, " "), "un septillón");
        assert_eq!(nsl.number_to_spanish(1000000000000000000000000000, " "), "mil cuatrillones");
        assert_eq!(nss.number_to_spanish(1000000000000000000000000000, " "), "un octillón");
        assert_eq!(nsl.number_to_spanish(1000000000000000000000000000000, " "), "un quintillón");
        assert_eq!(nss.number_to_spanish(1000000000000000000000000000000, " "), "un nonillón");

        assert_eq!(nsl.number_to_spanish(0, " "), "cero");
        assert_eq!(nsl.number_to_spanish(7, " "), "siete");
        assert_eq!(nsl.number_to_spanish(19, " "), "diecinueve");
        assert_eq!(nsl.number_to_spanish(21, " "), "veintiuno");
        assert_eq!(nsl.number_to_spanish(26, " "), "veintiséis");
        assert_eq!(nsl.number_to_spanish(29, " "), "veintinueve");
        assert_eq!(nsl.number_to_spanish(31, " "), "treinta y uno");
        assert_eq!(nsl.number_to_spanish(40, " "), "cuarenta");
        assert_eq!(nsl.number_to_spanish(56, " "), "cincuenta y seis");
        assert_eq!(nsl.number_to_spanish(99, " "), "noventa y nueve");
        assert_eq!(nsl.number_to_spanish(100, " "), "cien");
        assert_eq!(nsl.number_to_spanish(101, " "), "ciento uno");
        assert_eq!(nsl.number_to_spanish(120, " "), "ciento veinte");
        assert_eq!(nsl.number_to_spanish(456, " "), "cuatrocientos cincuenta y seis");  
        assert_eq!(nsl.number_to_spanish(999, " "), "novecientos noventa y nueve");
        assert_eq!(nsl.number_to_spanish(1000, " "), "mil");
        assert_eq!(nsl.number_to_spanish(1001, " "), "mil uno");
        assert_eq!(nsl.number_to_spanish(1456, " "), "mil cuatrocientos cincuenta y seis");  
        assert_eq!(nsl.number_to_spanish(2456, " "), "dos mil cuatrocientos cincuenta y seis");  
        assert_eq!(nsl.number_to_spanish(100000, " "), "cien mil");  
        assert_eq!(nsl.number_to_spanish(345456, " "), "trescientos cuarenta y cinco mil cuatrocientos cincuenta y seis");  
        assert_eq!(nsl.number_to_spanish(999999, " "), "novecientos noventa y nueve mil novecientos noventa y nueve");  
        assert_eq!(nsl.number_to_spanish(1000001, " "), "un millón uno");
        assert_eq!(nsl.number_to_spanish(1200300400100, " "), "un billón doscientos mil trescientos millones cuatrocientos mil cien");
        assert_eq!(nsl.number_to_spanish(12003004001000, " "), "doce billones tres mil cuatro millones mil");
        assert_eq!(nsl.number_to_spanish(91928091829809, " "), "noventa y un billones novecientos veintiocho mil noventa y un millones ochocientos veintinueve mil ochocientos nueve");
        assert_eq!(nsl.number_to_spanish(120030040010010, " "), "ciento veinte billones treinta mil cuarenta millones diez mil diez");
        assert_eq!(nsl.number_to_spanish(120130340010410103, " "), "ciento veinte mil ciento treinta billones trescientos cuarenta mil diez millones cuatrocientos diez mil ciento tres");
        assert_eq!(nsl.number_to_spanish(1201303400104101030, " "), "un trillón doscientos un mil trescientos tres billones cuatrocientos mil ciento cuatro millones ciento un mil treinta");
        assert_eq!(nsl.number_to_spanish(1000000000000000000000000, " "), "un cuatrillón");
        assert_eq!(nsl.number_to_spanish(1201303400104101035678987, " "), "un cuatrillón doscientos un mil trescientos tres trillones cuatrocientos mil ciento cuatro billones ciento un mil treinta y cinco millones seiscientos setenta y ocho mil novecientos ochenta y siete");
        assert_eq!(nsl.number_to_spanish(1271313457184191135678987, " "), "un cuatrillón doscientos setenta y un mil trescientos trece trillones cuatrocientos cincuenta y siete mil ciento ochenta y cuatro billones ciento noventa y un mil ciento treinta y cinco millones seiscientos setenta y ocho mil novecientos ochenta y siete");
        assert_eq!(nsl.number_to_spanish(193127131345718419113567898, " "), "ciento noventa y tres cuatrillones ciento veintisiete mil ciento treinta y un trillones trescientos cuarenta y cinco mil setecientos dieciocho billones cuatrocientos diecinueve mil ciento trece millones quinientos sesenta y siete mil ochocientos noventa y ocho");
        assert_eq!(nsl.number_to_spanish(1931271313457184191135678980, " "), "mil novecientos treinta y un cuatrillones doscientos setenta y un mil trescientos trece trillones cuatrocientos cincuenta y siete mil ciento ochenta y cuatro billones ciento noventa y un mil ciento treinta y cinco millones seiscientos setenta y ocho mil novecientos ochenta");
        assert_eq!(nsl.number_to_spanish(31415926535897932384626433832795, " "), "treinta y un quintillones cuatrocientos quince mil novecientos veintiséis cuatrillones quinientos treinta y cinco mil ochocientos noventa y siete trillones novecientos treinta y dos mil trescientos ochenta y cuatro billones seiscientos veintiséis mil cuatrocientos treinta y tres millones ochocientos treinta y dos mil setecientos noventa y cinco");

        assert_eq!(nsl.number_to_spanish(31415926535897932384626433832795, ", "), "treinta y un quintillones, cuatrocientos quince mil novecientos veintiséis cuatrillones, quinientos treinta y cinco mil ochocientos noventa y siete trillones, novecientos treinta y dos mil trescientos ochenta y cuatro billones, seiscientos veintiséis mil cuatrocientos treinta y tres millones, ochocientos treinta y dos mil setecientos noventa y cinco");
        assert_eq!(nss.number_to_spanish(31415926535897932384626433832795, ", "), "treinta y un nonillones, cuatrocientos quince octillones, novecientos veintiséis septillones, quinientos treinta y cinco sextillones, ochocientos noventa y siete quintillones, novecientos treinta y dos cuatrillones, trescientos ochenta y cuatro trillones, seiscientos veintiséis billones, cuatrocientos treinta y tres millones, ochocientos treinta y dos mil setecientos noventa y cinco");
    }
}
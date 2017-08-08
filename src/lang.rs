extern crate rand;

use rand::Rng;
use std::fmt;

//En is always the source, but never the target, so not included
#[derive(Clone, Copy)]
pub enum Lang {
    Af,
    Ar,
    Az,
    Be,
    Bg,
    Bn,
    Ca,
    Cs,
    Cy,
    Da,
    De,
    Eo,
    Es,
    Et,
    Eu,
    Fa,
    Fi,
    Fr,
    Ga,
    Gl,
    Gu,
    Hi,
    Hr,
    Ht,
    Hu,
    Id,
    Is,
    It,
    Iw,
    Ja,
    Ka,
    Kn,
    Ko,
    La,
    Lt,
    Lv,
    Mk,
    Ms,
    Mt,
    Nl,
    No,
    Pl,
    Pt,
    Ro,
    Ru,
    Sk,
    Sl,
    Sq,
    Sr,
    Sw,
    Sv,
    Ta,
    Te,
    Th,
    Tl,
    Tr,
    Uk,
    Ur,
    Vi,
    Yi,
}

impl Lang {
    pub fn random() -> Lang {
        let langs: [Lang; 60] = [
            Lang::Af,
            Lang::Ar,
            Lang::Az,
            Lang::Be,
            Lang::Bg,
            Lang::Bn,
            Lang::Ca,
            Lang::Cs,
            Lang::Cy,
            Lang::Da,
            Lang::De,
            Lang::Eo,
            Lang::Es,
            Lang::Et,
            Lang::Eu,
            Lang::Fa,
            Lang::Fi,
            Lang::Fr,
            Lang::Ga,
            Lang::Gl,
            Lang::Gu,
            Lang::Hi,
            Lang::Hr,
            Lang::Ht,
            Lang::Hu,
            Lang::Id,
            Lang::Is,
            Lang::It,
            Lang::Iw,
            Lang::Ja,
            Lang::Ka,
            Lang::Kn,
            Lang::Ko,
            Lang::La,
            Lang::Lt,
            Lang::Lv,
            Lang::Mk,
            Lang::Ms,
            Lang::Mt,
            Lang::Nl,
            Lang::No,
            Lang::Pl,
            Lang::Pt,
            Lang::Ro,
            Lang::Ru,
            Lang::Sk,
            Lang::Sl,
            Lang::Sq,
            Lang::Sr,
            Lang::Sw,
            Lang::Sv,
            Lang::Ta,
            Lang::Te,
            Lang::Th,
            Lang::Tl,
            Lang::Tr,
            Lang::Uk,
            Lang::Ur,
            Lang::Vi,
            Lang::Yi,
        ];

        *rand::thread_rng().choose(&langs).unwrap()
    }

    pub fn short(&self) -> String {
        match *self {
            Lang::Af => "af".to_owned(),
            Lang::Ar => "ar".to_owned(),
            Lang::Az => "az".to_owned(),
            Lang::Be => "be".to_owned(),
            Lang::Bg => "bg".to_owned(),
            Lang::Bn => "bn".to_owned(),
            Lang::Ca => "ca".to_owned(),
            Lang::Cs => "cs".to_owned(),
            Lang::Cy => "cy".to_owned(),
            Lang::Da => "da".to_owned(),
            Lang::De => "de".to_owned(),
            Lang::Eo => "eo".to_owned(),
            Lang::Es => "es".to_owned(),
            Lang::Et => "et".to_owned(),
            Lang::Eu => "eu".to_owned(),
            Lang::Fa => "fa".to_owned(),
            Lang::Fi => "fi".to_owned(),
            Lang::Fr => "fr".to_owned(),
            Lang::Ga => "ga".to_owned(),
            Lang::Gl => "gl".to_owned(),
            Lang::Gu => "gu".to_owned(),
            Lang::Hi => "hi".to_owned(),
            Lang::Hr => "hr".to_owned(),
            Lang::Ht => "ht".to_owned(),
            Lang::Hu => "hu".to_owned(),
            Lang::Id => "id".to_owned(),
            Lang::Is => "is".to_owned(),
            Lang::It => "it".to_owned(),
            Lang::Iw => "iw".to_owned(),
            Lang::Ja => "ja".to_owned(),
            Lang::Ka => "ka".to_owned(),
            Lang::Kn => "kn".to_owned(),
            Lang::Ko => "ko".to_owned(),
            Lang::La => "la".to_owned(),
            Lang::Lt => "lt".to_owned(),
            Lang::Lv => "lv".to_owned(),
            Lang::Mk => "mk".to_owned(),
            Lang::Ms => "ms".to_owned(),
            Lang::Mt => "mt".to_owned(),
            Lang::Nl => "nl".to_owned(),
            Lang::No => "no".to_owned(),
            Lang::Pl => "pl".to_owned(),
            Lang::Pt => "pt".to_owned(),
            Lang::Ro => "ro".to_owned(),
            Lang::Ru => "ru".to_owned(),
            Lang::Sk => "sk".to_owned(),
            Lang::Sl => "sl".to_owned(),
            Lang::Sq => "sq".to_owned(),
            Lang::Sr => "sr".to_owned(),
            Lang::Sw => "sw".to_owned(),
            Lang::Sv => "sv".to_owned(),
            Lang::Ta => "ta".to_owned(),
            Lang::Te => "te".to_owned(),
            Lang::Th => "th".to_owned(),
            Lang::Tl => "tl".to_owned(),
            Lang::Tr => "tr".to_owned(),
            Lang::Uk => "uk".to_owned(),
            Lang::Ur => "ur".to_owned(),
            Lang::Vi => "vi".to_owned(),
            Lang::Yi => "yi".to_owned(),
        }
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Lang::Af => "Afrikaans",
                Lang::Ar => "Arabic",
                Lang::Az => "Azerbaijani",
                Lang::Be => "Belarusian",
                Lang::Bg => "Bulgarian",
                Lang::Bn => "Bengali",
                Lang::Ca => "Catalan",
                Lang::Cs => "Czech",
                Lang::Cy => "Welsh",
                Lang::Da => "Danish",
                Lang::De => "German",
                Lang::Eo => "Esperanto",
                Lang::Es => "Spanish",
                Lang::Et => "Estonian",
                Lang::Eu => "Basque",
                Lang::Fa => "Farsi",
                Lang::Fi => "Finnish",
                Lang::Fr => "French",
                Lang::Ga => "Irish",
                Lang::Gl => "Galician",
                Lang::Gu => "Gujarati",
                Lang::Hi => "Hindi",
                Lang::Hr => "Croatian",
                Lang::Ht => "Haitian Creole",
                Lang::Hu => "Hungarian",
                Lang::Id => "Indonesian",
                Lang::Is => "Icelandic",
                Lang::It => "Italian",
                Lang::Iw => "Hebrew",
                Lang::Ja => "Japanese",
                Lang::Ka => "Georgian",
                Lang::Kn => "Kannada",
                Lang::Ko => "Korean",
                Lang::La => "Latin",
                Lang::Lt => "Lithuanian",
                Lang::Lv => "Latvian",
                Lang::Mk => "Macedonian",
                Lang::Ms => "Malay",
                Lang::Mt => "Maltese",
                Lang::Nl => "Dutch",
                Lang::No => "Norwegian",
                Lang::Pl => "Polish",
                Lang::Pt => "Portuguese",
                Lang::Ro => "Romanian",
                Lang::Ru => "Russian",
                Lang::Sk => "Slovak",
                Lang::Sl => "Slovenian",
                Lang::Sq => "Albanian",
                Lang::Sr => "Serbian",
                Lang::Sw => "Swahili",
                Lang::Sv => "Swedish",
                Lang::Ta => "Tamil",
                Lang::Te => "Telugu",
                Lang::Th => "Thai",
                Lang::Tl => "Tagalog",
                Lang::Tr => "Turkish",
                Lang::Uk => "Ukrainian",
                Lang::Ur => "Urdu",
                Lang::Vi => "Vietnamese",
                Lang::Yi => "Yiddish",
            }
        )
    }
}

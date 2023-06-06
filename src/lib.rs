#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use whichlang::{detect_language as detect_vendor, Lang as Lang_vendor};

#[napi(string_enum)]
/// ISO language name
pub enum Language {
  Arabic,
  Dutch,
  English,
  French,
  German,
  Hindi,
  Italian,
  Japanese,
  Korean,
  Mandarin,
  Portuguese,
  Russian,
  Spanish,
  Swedish,
  Turkish,
  Vietnamese,
}

impl From<Lang_vendor> for Language {
  fn from(val: Lang_vendor) -> Self {
    match val {
      Lang_vendor::Ara => Language::Arabic,
      Lang_vendor::Nld => Language::Dutch,
      Lang_vendor::Eng => Language::English,
      Lang_vendor::Fra => Language::French,
      Lang_vendor::Deu => Language::German,
      Lang_vendor::Hin => Language::Hindi,
      Lang_vendor::Ita => Language::Italian,
      Lang_vendor::Jpn => Language::Japanese,
      Lang_vendor::Kor => Language::Korean,
      Lang_vendor::Cmn => Language::Mandarin,
      Lang_vendor::Por => Language::Portuguese,
      Lang_vendor::Rus => Language::Russian,
      Lang_vendor::Spa => Language::Spanish,
      Lang_vendor::Swe => Language::Swedish,
      Lang_vendor::Tur => Language::Turkish,
      Lang_vendor::Vie => Language::Vietnamese,
    }
  }
}

#[napi(string_enum)]
/// three-letter code for the language
pub enum Lang {
  Ara,
  Cmn,
  Deu,
  Eng,
  Fra,
  Hin,
  Ita,
  Jpn,
  Kor,
  Nld,
  Por,
  Rus,
  Spa,
  Swe,
  Tur,
  Vie,
}

pub trait ToLang {
  fn to_lang(self) -> Lang;
}

impl ToLang for Lang_vendor {
  fn to_lang(self) -> Lang {
    match self {
      Self::Ara => Lang::Ara,
      Self::Cmn => Lang::Cmn,
      Self::Deu => Lang::Deu,
      Self::Eng => Lang::Eng,
      Self::Fra => Lang::Fra,
      Self::Hin => Lang::Hin,
      Self::Ita => Lang::Ita,
      Self::Jpn => Lang::Jpn,
      Self::Kor => Lang::Kor,
      Self::Nld => Lang::Nld,
      Self::Por => Lang::Por,
      Self::Rus => Lang::Rus,
      Self::Spa => Lang::Spa,
      Self::Swe => Lang::Swe,
      Self::Tur => Lang::Tur,
      Self::Vie => Lang::Vie,
    }
  }
}

#[napi(js_name = "detectLanguage")]
/// detect language and return `Language`
pub fn detect_language(text: String) -> Language {
  detect_vendor(&text).into()
}

#[napi(js_name = "detectLang")]
/// detect language and return `Lang`
pub fn detect_lang(text: String) -> Lang {
  detect_vendor(&text).to_lang()
}

#[napi(string_enum)]
pub enum LangISO6391 {
  AR,
  NL,
  EN,
  FR,
  DE,
  IT,
  JA,
  KO,
  PT,
  RU,
  ES,
  SV,
  HI,
  TR,
  VI,
  ZH,
}

pub trait ToISO6391 {
  fn to_iso6391(self) -> LangISO6391;
}

impl ToISO6391 for Lang_vendor {
  fn to_iso6391(self) -> LangISO6391 {
    match self {
      Self::Ara => LangISO6391::AR,
      Self::Cmn => LangISO6391::ZH,
      Self::Deu => LangISO6391::DE,
      Self::Eng => LangISO6391::EN,
      Self::Fra => LangISO6391::FR,
      Self::Hin => LangISO6391::HI,
      Self::Ita => LangISO6391::IT,
      Self::Jpn => LangISO6391::JA,
      Self::Kor => LangISO6391::KO,
      Self::Nld => LangISO6391::NL,
      Self::Por => LangISO6391::PT,
      Self::Rus => LangISO6391::RU,
      Self::Spa => LangISO6391::ES,
      Self::Swe => LangISO6391::SV,
      Self::Tur => LangISO6391::TR,
      Self::Vie => LangISO6391::VI,
    }
  }
}

#[napi(js_name = "detectLangISO6391")]
/// detect language and return `LangISO6391`
pub fn detect_lang_iso6391(text: String) -> LangISO6391 {
  detect_vendor(&text).to_iso6391()
}

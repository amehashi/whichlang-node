#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use whichlang::{detect_language, Lang as Lang_vendor};

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

impl Into<Language> for Lang_vendor {
  fn into(self) -> Language {
    match self {
      Self::Ara => Language::Arabic,
      Self::Nld => Language::Dutch,
      Self::Eng => Language::English,
      Self::Fra => Language::French,
      Self::Deu => Language::German,
      Self::Hin => Language::Hindi,
      Self::Ita => Language::Italian,
      Self::Jpn => Language::Japanese,
      Self::Kor => Language::Korean,
      Self::Cmn => Language::Mandarin,
      Self::Por => Language::Portuguese,
      Self::Rus => Language::Russian,
      Self::Spa => Language::Spanish,
      Self::Swe => Language::Swedish,
      Self::Tur => Language::Turkish,
      Self::Vie => Language::Vietnamese,
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

#[napi(js_name = "detectLang")]
pub fn detect_lang(text: String, three_letters: Option<bool>) -> Either<Lang, Language> {
  match three_letters {
    Some(true) => Either::A(detect_language(&text).to_lang()),
    _ => Either::B(detect_language(&text).into()),
  }
}

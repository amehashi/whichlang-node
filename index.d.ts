/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/** ISO language name */
export const enum Language {
  Arabic = 'Arabic',
  Dutch = 'Dutch',
  English = 'English',
  French = 'French',
  German = 'German',
  Hindi = 'Hindi',
  Italian = 'Italian',
  Japanese = 'Japanese',
  Korean = 'Korean',
  Mandarin = 'Mandarin',
  Portuguese = 'Portuguese',
  Russian = 'Russian',
  Spanish = 'Spanish',
  Swedish = 'Swedish',
  Turkish = 'Turkish',
  Vietnamese = 'Vietnamese',
}
/** three-letter code for the language */
export const enum Lang {
  Ara = 'Ara',
  Cmn = 'Cmn',
  Deu = 'Deu',
  Eng = 'Eng',
  Fra = 'Fra',
  Hin = 'Hin',
  Ita = 'Ita',
  Jpn = 'Jpn',
  Kor = 'Kor',
  Nld = 'Nld',
  Por = 'Por',
  Rus = 'Rus',
  Spa = 'Spa',
  Swe = 'Swe',
  Tur = 'Tur',
  Vie = 'Vie',
}
/** detect language and return `Language` */
export function detectLanguage(text: string): Language
/** detect language and return `Lang` */
export function detectLang(text: string): Lang
export const enum LangISO6391 {
  AR = 'AR',
  NL = 'NL',
  EN = 'EN',
  FR = 'FR',
  DE = 'DE',
  IT = 'IT',
  JA = 'JA',
  KO = 'KO',
  PT = 'PT',
  RU = 'RU',
  ES = 'ES',
  SV = 'SV',
  HI = 'HI',
  TR = 'TR',
  VI = 'VI',
  ZH = 'ZH',
}
/** detect language and return `LangISO6391` */
export function detectLangISO6391(text: string): LangISO6391

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
export function detectLang(text: string, threeLetters?: boolean | undefined | null): Lang | Language

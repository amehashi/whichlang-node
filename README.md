# `whichlang-node`

![https://github.com/amehashi/whichlang-node/actions](https://github.com/amehashi/whichlang-node/workflows/CI/badge.svg)

[whichlang](https://github.com/quickwit-oss/whichlang) binding for Node.js.

## Install this package

```shell
yarn add whichlang-node
pnpm add whichlang-node
npm install whichlang-node
```

## Support matrix

### Operating Systems

|                  | node14 | node16 | node18 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      |
| Android armv7    | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

## API

```ts
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
```

## Usage

```js
import { detectLanguage, Language } from 'whichlang-node'

console.assert(detectLanguage('ネムルバカ') === Language.Japanese)
```

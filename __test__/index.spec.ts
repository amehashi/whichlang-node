import test from 'ava'

import { detectLang, detectLanguage, detectLangISO6391, Language, Lang, LangISO6391 } from '../index'

const fixture = [
  ['见到你很高兴', Language.Mandarin, Lang.Cmn, LangISO6391.ZH],
  ['hello world', Language.English, Lang.Eng, LangISO6391.EN],
  ['ネムルバカ', Language.Japanese, Lang.Jpn, LangISO6391.JA],
] as const

for (const [input, expected, expectedThreeLetters, iso6391] of fixture) {
  test(`detect ${expected}`, (t) => {
    t.is(detectLanguage(input), expected)
    t.is(detectLang(input), expectedThreeLetters)
    t.is(detectLangISO6391(input), iso6391)
  })
}

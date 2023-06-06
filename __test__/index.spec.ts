import test from 'ava'

import { detectLang, detectLanguage, Language, Lang } from '../index'

const fixture = [
  ['见到你很高兴', Language.Mandarin, Lang.Cmn],
  ['hello world', Language.English, Lang.Eng],
  ['ネムルバカ', Language.Japanese, Lang.Jpn],
] as const

for (const [input, expected, expectedThreeLetters] of fixture) {
  test(`detect ${expected}`, (t) => {
    t.is(detectLanguage(input), expected)
    t.is(detectLang(input), expectedThreeLetters)
  })
}

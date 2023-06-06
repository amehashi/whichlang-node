import b from 'benny'
import { detectLang as detectLangWhatlang } from 'whatlang-node'

import { detectLang } from '../index'

async function run() {
  await b.suite(
    'detect lang',

    b.add('whichlang node', () => {
      detectLang('见到你很高兴')
    }),

    b.add('whatlang node', () => {
      detectLangWhatlang('见到你很高兴')
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})

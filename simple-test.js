const { detectLang } = require('./index')

console.assert(detectLang('ネムルバカ') === 'Japanese', 'japanese')

console.info('Simple test passed')

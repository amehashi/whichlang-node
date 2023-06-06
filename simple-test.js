const { detectLanguage } = require('./index')

console.assert(detectLanguage('ネムルバカ') === 'Japanese', 'japanese')

console.info('Simple test passed')

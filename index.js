let addon = require('./index.node');

console.log(`index.js: addon = '${JSON.stringify(addon)}'`);
console.log(`index.js: addon.hello = '${addon.hello}'`);

console.log(`index.js: addon.myFunc = '${addon.myFunc}'`);
console.log(`index.js: addon.myFunc() = '${addon.myFunc()}'`);


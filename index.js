let addon = require('./index.node');

console.log(`index.js: addon = '${JSON.stringify(addon)}'`);
console.log(`index.js: addon.hello = '${addon.hello}'`);

console.log(`index.js: addon.say_hello = '${addon.say_hello}'`);
console.log(`index.js: addon.say_hello() = '${addon.say_hello()}'`);

console.log(`index.js: addon.add = '${addon.add}'`);
console.log(`index.js: addon.add(3,4) = '${addon.add(3,4)}'`);


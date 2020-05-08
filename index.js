let addon = require('./index.node');

console.log(`index.js: addon = '${JSON.stringify(addon)}'`);
console.log(`index.js: addon.name = '${addon.name}'`);

console.log(`index.js: addon.say_hello = '${addon.say_hello}'`);
console.log(`index.js: addon.say_hello() = '${addon.say_hello()}'`);

console.log(`index.js: addon.add_doubles = '${addon.add_doubles}'`);
console.log(`index.js: addon.add_doubles(3.1,4.3) = '${addon.add_doubles(3.1,4.3)}'`);


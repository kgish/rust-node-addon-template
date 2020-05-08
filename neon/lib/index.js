const addon = require('../native/index.node');

console.log(addon.say_hello());

module.exports = addon;

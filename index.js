let addon = require('./index.node');

let n = +(process.argv[2] || 0)
if (Number.isNaN(n)) n = 0;

console.log(`n = ${n}`);

if (n === 0 || n === 1) {
    console.log(`index.js: addon = '${JSON.stringify(addon)}'`);
    console.log(`index.js: addon.name = '${addon.name}'`);
}

if (n === 0 || n === 2) {
    console.log(`index.js: addon.say_hello = '${addon.say_hello}'`);
    console.log(`index.js: addon.say_hello() => '${addon.say_hello()}'`);
}

if (n === 0 || n === 3) {
    console.log(`index.js: addon.add_doubles = '${addon.add_doubles}'`);
    console.log(`index.js: addon.add_doubles(3.1,4.3) => '${addon.add_doubles(3.1, 4.3)}'`);
}

if (n === 0 || n === 4) {
    const message = 'This is a message from the javascript world!';
    console.log(`index.js: addon.send_message = '${addon.send_message}'`);
    console.log(`index.js: addon.send_message('${message}') => '${addon.send_message(message)}'`);
}

if (n === 0 || n === 5) {
    console.log(`index.js: addon.fibonacci = '${addon.fibonacci}'`);
    const n = 80;
    addon.fibonacci(n).then(
        result => console.log(`index.js: addon.fibonacci(${n}) => OK '${result}'`),
        error => console.log(`index.js: addon.fibonacci(${n}) => NOK '${error}'`),
    );
}


const addon = require('../native/index.node');

let n = +(process.argv[2] || 0);
if (Number.isNaN(n)) n = 0;

console.log(`n = ${n}`);
console.log(`index.js: addon = '${JSON.stringify(addon)}'`);

if (n === 0 || n === 2) {
  console.log(`index.js: sayHello = '${addon.sayHello}'`);
  console.log(`index.js: sayHello() => '${addon.sayHello()}'`);
}

if (n === 0 || n === 3) {
  console.log(`index.js: addNumbers = '${addon.addNumbers}'`);
  console.log(`index.js: addNumbers(3.1,4.3) => '${addon.addNumbers(3.1, 4.3)}'`);
}

if (n === 0 || n === 4) {
  const message = 'This is a message from the javascript world!';
  console.log(`index.js: sendMessage = '${addon.sendMessage}'`);
  console.log(`index.js: sendMessage('${message}') => '${addon.sendMessage(message)}'`);
}

if (n === 0 || n === 1) {
  console.log(`index.js: getUser = '${addon.getUser}'`);
  console.log(`index.js: getUser() => '${JSON.stringify(addon.getUser())}'`);
}

if (n === 0 || n === 5) {
  console.log(`index.js: fibonacci = '${addon.fibonacci}'`);
  const x = 100000;
  addon.fibonacci(x, (err, result) => {
    if (err) {
      console.log(`index.js: fibonacci(${x}) => NOK '${err}'`)
    } else {
      console.log(`index.js: fibonacci(${x}) => OK '${result}' (${typeof result})`);
    }
  });

  console.log(`index.js: computing fibonacci(${x}) in background thread...`);
  console.log('index.js: main thread is still responsive!');
}

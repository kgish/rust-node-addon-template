const addon = require('../native/index.node');

let n = +(process.argv[2] || 0);
if (Number.isNaN(n)) n = 0;

console.log(`n = ${n}`);
console.log(`NODE: addon = '${JSON.stringify(addon)}'`);

if (n === 0 || n === 2) {
  console.log(`NODE: sayHello = '${addon.sayHello}'`);
  console.log(`NODE: sayHello() => '${addon.sayHello()}'`);
}

if (n === 0 || n === 3) {
  console.log(`NODE: addNumbers = '${addon.addNumbers}'`);
  console.log(`NODE: addNumbers(3.1,4.3) => '${addon.addNumbers(3.1, 4.3)}'`);
}

if (n === 0 || n === 4) {
  const message = 'This is a message from the javascript world!';
  console.log(`NODE: sendMessage = '${addon.sendMessage}'`);
  console.log(`NODE: sendMessage('${message}') => '${addon.sendMessage(message)}'`);
}

if (n === 0 || n === 1) {
  console.log(`NODE: getUser = '${addon.getUser}'`);
  console.log(`NODE: getUser() => '${JSON.stringify(addon.getUser())}'`);
}

if (n === 0 || n === 5) {
  console.log(`NODE: fibonacci = '${addon.fibonacci}'`);
  const x = 100000;
  addon.fibonacci(x, (err, result) => {
    if (err) {
      console.log(`NODE: fibonacci(${x}) => NOK '${err}'`)
    } else {
      console.log(`NODE: fibonacci(${x}) => OK '${result}' (${typeof result})`);
    }
  });

  console.log(`NODE: computing fibonacci(${x}) in background thread...`);
  console.log('NODE: main thread is still responsive!');
}

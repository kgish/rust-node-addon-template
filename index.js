const addon = require('./index.node');

let n = +(process.argv[2] || 0);
if (Number.isNaN(n)) n = 0;

console.log(`n = ${n}`);
console.log(`NODE: addon = '${JSON.stringify(addon)}'`);

if (n === 0 || n === 1) {
  console.log(`NODE: sayHello = '${addon.sayHello}'`);
  console.log(`NODE: sayHello() => '${addon.sayHello()}'`);
}

if (n === 0 || n === 2) {
  const message = 'This is a message from the javascript world!';
  console.log(`NODE: sendMessage = '${addon.sendMessage}'`);
  console.log(`NODE: sendMessage('${message}') => '${addon.sendMessage(message)}'`);
}

if (n === 0 || n === 3) {
  console.log(`NODE: addNumbers = '${addon.addNumbers}'`);
  console.log(`NODE: addNumbers(3.1,4.3) => '${addon.addNumbers(3.1, 4.3)}'`);
}

if (n === 0 || n === 4) {
  // TODO
  console.log(`NODE: getUser = '${addon.getUser}'`);
  console.log(`NODE: getUser() => '${JSON.stringify(addon.getUser())}'`);
}

if (n === 0 || n === 5) {
  console.log(`NODE: fibonacci = '${addon.fibonacci}'`);
  const x = 80;
  // const x = 100000;
  addon.fibonacci(x).then(
    result => console.log(`NODE: fibonacci(${x}) => OK '${result}' (${typeof result})`),
    error => console.log(`NODE: fibonacci(${x}) => NOK '${error}'`)
  ).catch(
    error => console.log(`NODE: fibonacci(${x}) => NOK '${error}'`)
  );

  console.log(`NODE: computing fibonacci(${x}) in background thread...`);
  console.log('NODE: main thread is still responsive!');
}

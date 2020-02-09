var addon = require('../native');

console.log(addon.encode("test image", "example.png", "hidden_example.png"));
console.log(addon.decode("hidden_example.png"));

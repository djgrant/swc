// @moduleResolution: node12
// @module: node12
// @filename: /src/buzz.mts
// Extensionless relative path cjs import in an ES module
const foo = require("./foo"); // should error, should not ask for extension
export { };

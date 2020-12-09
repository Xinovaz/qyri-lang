var expr = require("./expr");
var con = require("./concatenate");


var expression = expr.parse("\"Sam \" + \"is \" +  \"cool\"");

console.log(expression);
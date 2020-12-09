{
	var type = require('./types');
}

Expression
  = head:Term tail:(_ ("+" / "-") _ Term)* {
      return tail.reduce(function(result, element) {
        if (element[1] === "+") { return result + element[3]; }
        if (element[1] === "-") { return result - element[3]; }
      }, head);
    }

Term
  = head:Factor tail:(_ ("*" / "/") _ Factor)* {
      return tail.reduce(function(result, element) {
        if (element[1] === "*") { return result * element[3]; }
        if (element[1] === "/") { return result / element[3]; }
      }, head);
    }

Factor
  = "(" _ expr:Expression _ ")" { return expr; }
  / integer



integer "_integer"
  = digits:number { return new type._integer(parseInt(digits.join(""))).canonical; }

float "_float"
  = digits:number?"."number { return new type._float(parseFloat(digits.join(""))).canonical; }



digit = [0-9]
number = digit+

space = " "
_ "whitespace"
  = [ \t\n\r]*
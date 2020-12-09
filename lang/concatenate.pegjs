concatenate
  = (left:string _ "+" _ right:string)+ { return type._string(left + right).canonical; }


string "_string"
  = words:"\""(word space?)*"\"" { return type._string(words.join("")).canonical; }
  / words:"\'"(word space?)*"\'" { return type._string(words.join("")).canonical; }

word = char+

char = [a-zA-Z0-9]
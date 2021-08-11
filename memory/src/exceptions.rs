// Built-in exceptions are just strings


pub const AttributeException: &str = "\"{}\" is not an attribute of {}";
pub const ExternalCodeException: &str = "unable to {} package \"{}\"";
pub const RecursionException: &str = "recursion limit reached";
pub const MapException: &str = "key \"{}\" not found in {}";
pub const StackOverflowException: &str = "caused the stack to overflow";
pub const IdentifierException: &str = "{} \"{}\" not found";
pub const SegmentationFaultException: &str = "segmentation fault";
pub const SyntaxException: &str = "invalid syntax";
pub const TypeException: &str = "{} is not of type {}";
pub const DivideByZeroException: &str = "cannot divide by zero";
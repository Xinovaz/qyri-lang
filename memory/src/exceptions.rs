// Built-in exceptions are just strings


pub const AttributeException: String = "\"{}\" is not an attribute of {}".to_string();
pub const ExternalCodeException: String = "unable to {} package \"{}\"".to_string();
pub const RecursionException: String = "recursion limit reached".to_string();
pub const MapException: String = "key \"{}\" not found in {}".to_string();
pub const StackOverflowException: String = "caused the stack to overflow".to_string();
pub const IdentifierException: String = "{} \"{}\" not found".to_string();
pub const SegmentationFaultException: String = "segmentation fault".to_string();
pub const SyntaxException: String = "invalid syntax".to_string();
pub const TypeException: String = "{} is not of type {}".to_string();
pub const DivideByZeroException: String = "cannot divide by zero".to_string();
//! Type system definitions for omnitype.

use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// A type variable used during type inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeVar(pub u32);

impl fmt::Display for TypeVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "T{}", self.0)
    }
}

/// Represents a type in the omnitype system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    /// The unknown type (used during inference)
    Unknown,
    
    /// The `None` type
    None,
    
    /// The `Any` type (top type)
    Any,
    
    /// Boolean type (true/false)
    Bool,
    
    /// Integer number type
    Int,
    
    /// Floating-point number type
    Float,
    
    /// Unicode string type
    Str,
    
    /// Binary data type
    Bytes,
    
    /// Homogeneous list/array type
    List(Box<Type>),
    
    /// Dictionary/map type with key and value types
    Dict(Box<Type>, Box<Type>),  // key type, value type
    
    /// Fixed-size heterogeneous sequence type
    Tuple(Vec<Type>),
    
    /// Unordered collection of unique elements
    Set(Box<Type>),
    
    /// Function type with parameter and return types
    Function {
        /// List of parameter types
        params: Vec<Type>,
        /// Return type
        returns: Box<Type>,
    },
    
    /// Union type representing one of several possible types (T1 | T2 | ...)
    Union(Vec<Type>),
    
    /// Type variable used during type inference
    Var(TypeVar),
    
    /// Named type (e.g., user-defined class or type alias)
    Named(String),
    
    /// Generic type with type parameters
    Generic {
        /// Name of the generic type
        name: String,
        /// Type parameters
        params: Vec<Type>,
    },
}

impl Default for Type {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Unknown => write!(f, "Unknown"),
            Type::None => write!(f, "None"),
            Type::Any => write!(f, "Any"),
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Str => write!(f, "str"),
            Type::Bytes => write!(f, "bytes"),
            Type::List(inner) => write!(f, "List[{}]", inner),
            Type::Dict(k, v) => write!(f, "Dict[{}, {}]", k, v),
            Type::Tuple(items) => {
                let items_str = items
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "Tuple[{}]", items_str)
            }
            Type::Set(inner) => write!(f, "Set[{}]", inner),
            Type::Function { params, returns } => {
                let params_str = params
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "Callable[[{}], {}]", params_str, returns)
            }
            Type::Union(types) => {
                let types_str = types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" | ");
                write!(f, "{}", types_str)
            }
            Type::Var(var) => write!(f, "{}", var),
            Type::Named(name) => write!(f, "{}", name),
            Type::Generic { name, params } => {
                let params_str = params
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}[{}]", name, params_str)
            }
        }
    }
}

/// Type environment that maps variable names to their types.
#[derive(Debug, Default, Clone)]
pub struct TypeEnv {
    bindings: HashMap<String, Type>,
    parent: Option<Box<TypeEnv>>,
}

impl TypeEnv {
    /// Creates a new empty type environment.
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None,
        }
    }
    
    /// Creates a new nested type environment.
    pub fn nested(env: TypeEnv) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(Box::new(env)),
        }
    }
    
    /// Looks up a variable in the environment.
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.bindings.get(name).or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.lookup(name))
        })
    }
    
    /// Binds a variable to a type in the current scope.
    pub fn bind(&mut self, name: String, ty: Type) -> Option<Type> {
        self.bindings.insert(name, ty)
    }
    
    /// Returns the parent environment, if any.
    pub fn parent(&self) -> Option<&TypeEnv> {
        self.parent.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_display() {
        assert_eq!(Type::Int.to_string(), "int");
        assert_eq!(Type::List(Box::new(Type::Int)).to_string(), "List[int]");
        assert_eq!(
            Type::Dict(Box::new(Type::Str), Box::new(Type::Int)).to_string(),
            "Dict[str, int]"
        );
        assert_eq!(
            Type::Function {
                params: vec![Type::Int, Type::Str],
                returns: Box::new(Type::Bool)
            }
            .to_string(),
            "Callable[[int, str], bool]"
        );
    }
    
    #[test]
    fn test_type_env() {
        let mut env = TypeEnv::new();
        env.bind("x".to_string(), Type::Int);
        
        assert_eq!(env.lookup("x"), Some(&Type::Int));
        assert_eq!(env.lookup("y"), None);
        
        let mut inner_env = TypeEnv::nested(env);
        inner_env.bind("y".to_string(), Type::Str);
        
        assert_eq!(inner_env.lookup("x"), Some(&Type::Int));
        assert_eq!(inner_env.lookup("y"), Some(&Type::Str));
    }
}

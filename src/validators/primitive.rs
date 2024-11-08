use crate::{Error, Validator, Value};

/// Match if string match predicate.
pub fn string<F>(predicate: F) -> impl Validator
where
    F: Fn(&String) -> Result<(), String> + 'static,
{
    PrimitiveValidator {
        typename: String::from("string"),
        extract: |val| val.as_str().map(String::from),
        predicate,
    }
}

/// Match if null.
#[must_use]
pub fn null() -> impl Validator {
    PrimitiveValidator {
        typename: String::from("null"),
        extract: serde_json::Value::as_null,
        predicate: |()| Ok(()),
    }
}

/// Match if bool match predicate.
pub fn bool<F>(predicate: F) -> impl Validator
where
    F: Fn(&bool) -> Result<(), String> + 'static,
{
    PrimitiveValidator {
        typename: String::from("bool"),
        extract: serde_json::Value::as_bool,
        predicate,
    }
}

/// Match if number match predicate.
pub fn i64<F>(predicate: F) -> impl Validator
where
    F: Fn(&i64) -> Result<(), String> + 'static,
{
    PrimitiveValidator {
        typename: String::from("i64"),
        extract: serde_json::Value::as_i64,
        predicate,
    }
}

/// Match if number match predicate.
pub fn u64<F>(predicate: F) -> impl Validator
where
    F: Fn(&u64) -> Result<(), String> + 'static,
{
    PrimitiveValidator {
        typename: String::from("u64"),
        extract: serde_json::Value::as_u64,
        predicate,
    }
}

/// Match if number match predicate.
pub fn f64<F>(predicate: F) -> impl Validator
where
    F: Fn(&f64) -> Result<(), String> + 'static,
{
    PrimitiveValidator {
        typename: String::from("f64"),
        extract: serde_json::Value::as_f64,
        predicate,
    }
}

struct PrimitiveValidator<T, F, G>
where
    F: Fn(&T) -> Result<(), String>,
    G: Fn(&Value) -> Option<T>,
{
    typename: String,
    extract: G,
    predicate: F,
}

impl<T, F, G> Validator for PrimitiveValidator<T, F, G>
where
    F: Fn(&T) -> Result<(), String>,
    G: Fn(&Value) -> Option<T>,
{
    fn validate<'a>(&self, value: &'a Value) -> Result<(), Error<'a>> {
        let val = (self.extract)(value)
            .ok_or_else(|| Error::InvalidType(value, self.typename.clone()))?;

        (self.predicate)(&val).map_err(|msg| Error::InvalidValue(value, msg))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Validator, Value};

    #[test]
    fn string() {
        let validator = super::string(|_| Ok(()));

        assert_eq!(Ok(()), validator.validate(&Value::String("ok".to_string())));
    }

    #[test]
    fn string_invalid_value() {
        let validator = super::string(|_| Err(String::from("error message")));

        assert!(matches!(
            validator.validate(&Value::String(String::new())),
            Err(Error::InvalidValue(_, _))
        ));
    }

    #[test]
    fn string_invalid_type() {
        let validator = super::string(|_| Ok(()));

        assert!(matches!(
            validator.validate(&Value::Null),
            Err(Error::InvalidType(_, _))
        ));
    }

    #[test]
    fn null() {
        let validator = super::null();

        assert_eq!(Ok(()), validator.validate(&Value::Null));
    }

    #[test]
    fn i64() {
        let validator = super::i64(|_| Ok(()));

        assert_eq!(Ok(()), validator.validate(&serde_json::json!(4)));
    }
}

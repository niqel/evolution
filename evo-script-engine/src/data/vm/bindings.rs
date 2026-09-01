use std::collections::HashMap;

use evo_values::{OwnedValue, Value};

use crate::data::failures::ExternalCapabilityFailure;
use crate::data::semantic::SignatureSymbol;

pub(crate) type ExternalCapability =
    for<'value> fn(&'value [Value<'value>]) -> Result<OwnedValue, ExternalCapabilityFailure>;

pub(crate) struct ApplicationBindings {
    pub(crate) capabilities: HashMap<SignatureSymbol, ExternalCapability>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;
    use alloc::string::ToString;

    fn success_capability<'value>(
        values: &'value [Value<'value>],
    ) -> Result<OwnedValue, ExternalCapabilityFailure> {
        let _ = values;
        Ok(OwnedValue::Boolean(true))
    }

    fn failure_capability<'value>(
        _values: &'value [Value<'value>],
    ) -> Result<OwnedValue, ExternalCapabilityFailure> {
        Err(ExternalCapabilityFailure {
            code: Box::from("not_found"),
        })
    }

    fn echo_first_string_capability<'value>(
        values: &'value [Value<'value>],
    ) -> Result<OwnedValue, ExternalCapabilityFailure> {
        if let Some(Value::String(s)) = values.first() {
            Ok(OwnedValue::String(Box::from(*s)))
        } else {
            Err(ExternalCapabilityFailure {
                code: Box::from("invalid_argument"),
            })
        }
    }

    #[test]
    fn external_capability_exact_abi_invocation() {
        let cap: ExternalCapability = success_capability;
        let args: [Value<'_>; 0] = [];
        let res = cap(&args);
        match res {
            Ok(OwnedValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Boolean(true)"),
        }
    }

    #[test]
    fn external_capability_failure_abi() {
        let cap: ExternalCapability = failure_capability;
        let args: [Value<'_>; 0] = [];
        let res = cap(&args);
        match res {
            Err(err) => assert_eq!(&*err.code, "not_found"),
            _ => panic!("expected ExternalCapabilityFailure"),
        }
    }

    #[test]
    fn external_capability_borrowed_lifetime_and_owned_result() {
        let cap: ExternalCapability = echo_first_string_capability;
        let local_str = "borrowed_test_data".to_string();
        let value = Value::String(&local_str);
        let args = [value];

        let res = cap(&args);
        match res {
            Ok(OwnedValue::String(s)) => assert_eq!(&*s, "borrowed_test_data"),
            _ => panic!("expected OwnedValue::String"),
        }
    }

    #[test]
    fn application_bindings_contractual_lookup_and_superset() {
        let math_square = SignatureSymbol {
            module: "math".to_string(),
            name: "square".to_string(),
        };

        let fs_read = SignatureSymbol {
            module: "fs".to_string(),
            name: "read".to_string(),
        };

        let db_read = SignatureSymbol {
            module: "db".to_string(),
            name: "read".to_string(),
        };

        let mut map = HashMap::new();
        map.insert(
            SignatureSymbol {
                module: "math".to_string(),
                name: "square".to_string(),
            },
            success_capability as ExternalCapability,
        );
        map.insert(
            SignatureSymbol {
                module: "fs".to_string(),
                name: "read".to_string(),
            },
            failure_capability as ExternalCapability,
        );
        map.insert(
            SignatureSymbol {
                module: "db".to_string(),
                name: "read".to_string(),
            },
            echo_first_string_capability as ExternalCapability,
        );

        let bindings = ApplicationBindings { capabilities: map };

        assert_eq!(bindings.capabilities.len(), 3);

        // Lookup by SignatureSymbol
        let math_cap = bindings
            .capabilities
            .get(&math_square)
            .expect("math::square exists");
        let math_res = math_cap(&[]);
        match math_res {
            Ok(OwnedValue::Boolean(b)) => assert!(b),
            _ => panic!("expected Ok(Boolean)"),
        }

        // Distinct namespaces fs::read vs db::read
        let fs_cap = bindings
            .capabilities
            .get(&fs_read)
            .expect("fs::read exists");
        match fs_cap(&[]) {
            Err(err) => assert_eq!(&*err.code, "not_found"),
            _ => panic!("expected not_found"),
        }

        let db_cap = bindings
            .capabilities
            .get(&db_read)
            .expect("db::read exists");
        let test_arg = Value::String("hello");
        match db_cap(&[test_arg]) {
            Ok(OwnedValue::String(s)) => assert_eq!(&*s, "hello"),
            _ => panic!("expected OwnedValue::String(hello)"),
        }
    }
}

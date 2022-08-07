use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use huff_codegen::Codegen;
use huff_utils::prelude::*;

#[test]
fn constructs_valid_abi() {
    let constructor = MacroDefinition {
        name: "CONSTRUCTOR".to_string(),
        parameters: vec![],
        statements: vec![],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![]),
        outlined: false,
        test: false,
    };
    let contract = Contract {
        macros: vec![constructor],
        invocations: vec![],
        imports: vec![],
        constants: Rc::new(RefCell::new(vec![])),
        errors: vec![],
        functions: vec![],
        events: vec![],
        tables: vec![],
    };

    // Generate the abi from the contract
    let mut cg = Codegen::new();
    let abi = cg.abi_gen(contract, None).unwrap();
    assert_eq!(
        abi,
        Abi {
            constructor: Some(Constructor { inputs: vec![] }),
            functions: BTreeMap::new(),
            events: BTreeMap::new(),
            errors: BTreeMap::new(),
            receive: false,
            fallback: false
        }
    );
}

#[test]
fn missing_constructor_fails() {
    let _constructor = MacroDefinition {
        name: "CONSTRUCTOR".to_string(),
        parameters: vec![],
        statements: vec![],
        takes: 0,
        returns: 0,
        span: AstSpan(vec![]),
        outlined: false,
        test: false,
    };
    let contract = Contract {
        macros: vec![],
        invocations: vec![],
        imports: vec![],
        constants: Rc::new(RefCell::new(vec![])),
        errors: vec![],
        functions: vec![],
        events: vec![],
        tables: vec![],
    };

    // Generate the abi from the contract
    // This should fail since there's no constructor
    let mut cg = Codegen::new();
    let abi = cg.abi_gen(contract, None);
    assert!(abi.unwrap().constructor.is_none());
}

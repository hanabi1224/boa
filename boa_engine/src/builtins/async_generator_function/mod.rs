//! Boa's implementation of ECMAScript's `AsyncGeneratorFunction` object.
//!
//! More information:
//!  - [ECMAScript reference][spec]
//!
//! [spec]: https://tc39.es/ecma262/#sec-asyncgeneratorfunction-objects

use crate::{
    builtins::{function::BuiltInFunctionObject, BuiltInObject},
    context::intrinsics::{Intrinsics, StandardConstructor, StandardConstructors},
    object::{JsObject, PROTOTYPE},
    property::Attribute,
    symbol::JsSymbol,
    value::JsValue,
    Context, JsResult,
};
use boa_profiler::Profiler;

use super::{BuiltInBuilder, BuiltInConstructor, IntrinsicObject};

/// The internal representation of an `AsyncGeneratorFunction` object.
#[derive(Debug, Clone, Copy)]
pub struct AsyncGeneratorFunction;

impl IntrinsicObject for AsyncGeneratorFunction {
    fn init(intrinsics: &Intrinsics) {
        let _timer = Profiler::global().start_event(Self::NAME, "init");

        BuiltInBuilder::from_standard_constructor::<Self>(intrinsics)
            .inherits(Some(intrinsics.constructors().function().prototype()))
            .constructor_attributes(Attribute::CONFIGURABLE)
            .property(
                PROTOTYPE,
                intrinsics.objects().async_generator(),
                Attribute::CONFIGURABLE,
            )
            .property(
                JsSymbol::to_string_tag(),
                Self::NAME,
                Attribute::CONFIGURABLE,
            )
            .build();
    }

    fn get(intrinsics: &Intrinsics) -> JsObject {
        Self::STANDARD_CONSTRUCTOR(intrinsics.constructors()).constructor()
    }
}

impl BuiltInObject for AsyncGeneratorFunction {
    const NAME: &'static str = "AsyncGeneratorFunction";
}

impl BuiltInConstructor for AsyncGeneratorFunction {
    const LENGTH: usize = 1;

    const STANDARD_CONSTRUCTOR: fn(&StandardConstructors) -> &StandardConstructor =
        StandardConstructors::async_generator_function;

    /// `AsyncGeneratorFunction ( p1, p2, … , pn, body )`
    ///
    /// More information:
    ///  - [ECMAScript reference][spec]
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-asyncgeneratorfunction
    fn constructor(
        new_target: &JsValue,
        args: &[JsValue],
        context: &mut Context<'_>,
    ) -> JsResult<JsValue> {
        BuiltInFunctionObject::create_dynamic_function(new_target, args, true, true, context)
            .map(Into::into)
    }
}

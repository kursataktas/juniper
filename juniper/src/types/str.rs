//! GraphQL implementation for [`str`].
//!
//! [`str`]: primitive@std::str

use std::{rc::Rc, sync::Arc};

use futures::future;

use crate::{
    graphql,
    meta::MetaType,
    parser::{ParseError, ScalarToken},
    reflect, resolve, BoxFuture, ExecutionResult, Executor, Registry, ScalarValue, Selection,
};

impl<Info: ?Sized, S: ScalarValue> resolve::Type<Info, S> for str {
    fn meta<'r>(registry: &mut Registry<'r, S>, info: &Info) -> MetaType<'r, S>
    where
        S: 'r,
    {
        registry
            .build_scalar_type_unsized::<Self, _>(info)
            .into_meta()
    }
}

impl<Info: ?Sized> resolve::TypeName<Info> for str {
    fn type_name(_: &Info) -> &'static str {
        <Self as reflect::BaseType<()>>::NAME
    }
}

impl<Info, Ctx, S> resolve::Value<Info, Ctx, S> for str
where
    Info: ?Sized,
    Ctx: ?Sized,
    S: From<String>,
{
    fn resolve_value(
        &self,
        _: Option<&[Selection<'_, S>]>,
        _: &Info,
        _: &Executor<Ctx, S>,
    ) -> ExecutionResult<S> {
        // TODO: Remove redundant `.to_owned()` allocation by allowing
        //       `ScalarValue` creation from reference?
        Ok(graphql::Value::scalar(self.to_owned()))
    }
}

impl<Info, Ctx, S> resolve::ValueAsync<Info, Ctx, S> for str
where
    Info: ?Sized,
    Ctx: ?Sized,
    S: From<String> + Send,
{
    fn resolve_value_async<'r>(
        &'r self,
        _: Option<&'r [Selection<'_, S>]>,
        _: &'r Info,
        _: &'r Executor<Ctx, S>,
    ) -> BoxFuture<'r, ExecutionResult<S>> {
        // TODO: Remove redundant `.to_owned()` allocation by allowing
        //       `ScalarValue` creation from reference?
        Box::pin(future::ok(graphql::Value::scalar(self.to_owned())))
    }
}

impl<S> resolve::ToInputValue<S> for str
where
    S: From<String>,
{
    fn to_input_value(&self) -> graphql::InputValue<S> {
        graphql::InputValue::scalar(self.to_owned())
    }
}

impl<S: ScalarValue> resolve::InputValueAsRef<S> for str {
    type Error = String;

    fn try_from_input_value(v: &graphql::InputValue<S>) -> Result<&Self, Self::Error> {
        v.as_string_value()
            .ok_or_else(|| format!("Expected `String`, found: {}", v))
    }
}

impl<'inp, S: ScalarValue> resolve::InputValueAsBox<'inp, S> for str {
    type Error = String;

    fn try_from_input_value(v: &'inp graphql::InputValue<S>) -> Result<Box<Self>, Self::Error> {
        <str as resolve::InputValueAsRef<S>>::try_from_input_value(v).map(Into::into)
    }
}

impl<'inp, S: ScalarValue> resolve::InputValueAsArc<'inp, S> for str {
    type Error = String;

    fn try_from_input_value(v: &'inp graphql::InputValue<S>) -> Result<Arc<Self>, Self::Error> {
        <str as resolve::InputValueAsRef<S>>::try_from_input_value(v).map(Into::into)
    }
}

impl<'inp, S: ScalarValue> resolve::InputValueAsRc<'inp, S> for str {
    type Error = String;

    fn try_from_input_value(v: &'inp graphql::InputValue<S>) -> Result<Rc<Self>, Self::Error> {
        <str as resolve::InputValueAsRef<S>>::try_from_input_value(v).map(Into::into)
    }
}

impl<S> resolve::ScalarToken<S> for str
where
    String: resolve::ScalarToken<S>,
{
    fn parse_scalar_token(token: ScalarToken<'_>) -> Result<S, ParseError<'_>> {
        <String as resolve::ScalarToken<S>>::parse_scalar_token(token)
    }
}

/*
impl<'i, Info, S: 'i> graphql::InputType<'i, Info, S> for str
where
    Self: resolve::Type<Info, S> + resolve::ToInputValue<S> + resolve::InputValue<'i, S>,
    Info: ?Sized,
{
    fn assert_input_type() {}
}
*/

impl<S> graphql::OutputType<S> for str {
    fn assert_output_type() {}
}

impl<S> graphql::Scalar<S> for str {
    fn assert_scalar() {}
}

impl<S> reflect::BaseType<S> for str {
    const NAME: reflect::Type = <String as reflect::BaseType<S>>::NAME;
}

impl<S> reflect::BaseSubTypes<S> for str {
    const NAMES: reflect::Types = &[<Self as reflect::BaseType<S>>::NAME];
}

impl<S> reflect::WrappedType<S> for str {
    const VALUE: reflect::WrappedValue = reflect::wrap::SINGULAR;
}
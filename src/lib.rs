use std::{fmt::Display, marker::PhantomData};

use cgp::prelude::*;

#[derive_component(TxExecutorComponent, TxExecutorProvider<Rollup>)]
pub trait CanExecuteTx {
    fn do_it(&self, param: String) -> String;
}

// a helper bound on context, to attach Spec as assosiated type
pub trait HasType {
    type Spec: RollupSpec;
}

pub trait RollupSpec {
    fn specific() -> String;
}

pub struct RollupTxExecutor;

impl<Rollup> TxExecutorProvider<Rollup> for RollupTxExecutor
where
    Rollup: Display,
    // provider needs to call spec specific method
    Rollup: HasType,
{
    fn do_it(rollup: &Rollup, param: String) -> String {
        format!(
            "param: {param}, context {rollup} and from spec: {}",
            <Rollup as HasType>::Spec::specific()
        )
    }
}

// rollup is generic over spec
pub struct Rollup<S>(String, PhantomData<S>);

impl<S> Rollup<S> {
    pub fn new(v: &str) -> Self {
        Rollup(v.to_owned(), Default::default())
    }
}

impl<S> Display for Rollup<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

// rollup has a spec type attached because instead of implementing providers generic over spec we add a bound on context itself
impl<S: RollupSpec> HasType for Rollup<S> {
    type Spec = S;
}

// same components for rollup with any spec
impl<S> HasComponents for Rollup<S> {
    type Components = RollupComponents;
}

pub struct RollupComponents;

delegate_components! {
    RollupComponents {
        TxExecutorComponent: RollupTxExecutor
    }
}

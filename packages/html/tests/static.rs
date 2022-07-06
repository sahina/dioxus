use std::{any::Any, ops::Deref, ptr::addr_of};
//
use dioxus_core::prelude::*;

struct NodeBuilder<'a, P> {
    inner: Scope<'a>,
    parent: P,
}
impl NodeBuilder<'_, ()> {
    fn class(self, f: &str) -> Self {
        todo!()
    }

    fn build(self) {}
}

struct Base<'a> {
    inner: &'a ScopeState,
}

trait MyThing<T> {}

trait Buildable<P, O> {
    type Builder;
}

impl<'a, F, I> Buildable<(), NodeBuilder<'a, I>> for F
where
    F: Fn(&ScopeState) -> NodeBuilder<I>,
{
    type Builder = NodeBuilder<'static, I>;
}

impl<F, P> Buildable<P, Element<'static>> for F
where
    F: Fn(Scope<P>) -> Element,
    P: Properties,
{
    type Builder = P::Builder;
}

fn buildit<P: Properties, O, C: Buildable<P, O>>(f: C) -> C::Builder {
    todo!()
}

fn base(cx: &ScopeState) -> NodeBuilder<()> {
    todo!()
}

fn my_component(cx: Scope) -> Element {
    todo!()
}

fn dis_ambiguate(cx: Scope) {
    // Used by builder
    let builder = base(&cx);

    // Used by the macro
    let r = buildit(base).class("asda").build();
    let r = buildit(my_component).build();
}

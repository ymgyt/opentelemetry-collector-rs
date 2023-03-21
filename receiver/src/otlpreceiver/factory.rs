use crate::receiver;

const TYPE_STR: component::Type = component::Type("otlp");

pub fn new_factory() -> impl receiver::Factory {
    receiver::new_factory(TYPE_STR)
}
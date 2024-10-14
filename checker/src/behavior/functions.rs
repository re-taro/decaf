use std::collections::HashMap;

use source_map::Span;

use crate::{
    context::{facts::Facts, Context, ContextType},
    events::Event,
    types::{
        functions::SynthesizedParameters, poly_types::GenericTypeParameters, properties::Property,
        FunctionType, TypeStore,
    },
    CheckingData, Environment, FSResolver, FunctionId, Type, TypeId, VariableId,
};

#[derive(Copy, Clone, Debug, binary_serialize_derive::BinarySerializable)]
pub enum GetterSetterGeneratorOrNone {
    Setter,
    Getter,
    Generator,
    None,
}

pub enum FunctionKind2 {
    ArrowFunction {
        is_async: bool,
    },
    StatementFunction {
        is_async: bool,
        generator: bool,
    },
    ClassConstructor,
    Method {
        getter_setter_or_generator: GetterSetterGeneratorOrNone,
    },
}

/// TODO generalize for property registration...
pub trait RegisterBehavior {
    type Return;

    /// TODO lift T
    fn function<T: SynthesizableFunction, U: ContextType>(
        &self,
        func: &T,
        func_ty: FunctionType,
        environment: &mut Context<U>,
        types: &mut TypeStore,
    ) -> Self::Return;
}

pub struct RegisterAsType;

impl RegisterBehavior for RegisterAsType {
    type Return = TypeId;

    fn function<T: SynthesizableFunction, U: ContextType>(
        &self,
        func: &T,
        func_ty: FunctionType,
        environment: &mut Context<U>,
        types: &mut TypeStore,
    ) -> Self::Return {
        let id = func_ty.id;
        types.functions.insert(id, func_ty);
        let ty = types.register_type(crate::Type::Function(id, ThisValue::UseParent));
        environment.facts.events.push(Event::CreateObject {
            prototype: crate::events::PrototypeArgument::Function(id),
            referenced_in_scope_as: ty,
        });
        ty
    }
}

/// Because of hoisting
pub struct RegisterOnExisting(pub String);

impl RegisterBehavior for RegisterOnExisting {
    type Return = ();

    fn function<T: SynthesizableFunction, U: ContextType>(
        &self,
        func: &T,
        func_ty: FunctionType,
        environment: &mut Context<U>,
        types: &mut TypeStore,
    ) -> Self::Return {
        let id = func_ty.id;
        types.functions.insert(id, func_ty);
        let ty = types.register_type(crate::Type::Function(id, Default::default()));
        let variable_id = environment
            .variables
            .get(&self.0)
            .unwrap()
            .declared_at
            .clone();
        let variable_id = VariableId(variable_id.source, variable_id.start);
        environment
            .facts
            .variable_current_value
            .insert(variable_id, ty);
        environment.facts.events.push(Event::CreateObject {
            prototype: crate::events::PrototypeArgument::Function(id),
            referenced_in_scope_as: ty,
        });
    }
}

pub struct RegisterOnExistingObject;

impl RegisterBehavior for RegisterOnExistingObject {
    type Return = Property;

    fn function<T: SynthesizableFunction, U: ContextType>(
        &self,
        func: &T,
        func_ty: FunctionType,
        environment: &mut Context<U>,
        types: &mut TypeStore,
    ) -> Self::Return {
        match func.get_set_generator_or_none() {
            crate::GetterSetterGeneratorOrNone::Getter => Property::Getter(Box::new(func_ty)),
            crate::GetterSetterGeneratorOrNone::Setter => Property::Setter(Box::new(func_ty)),
            // TODO Generator
            crate::GetterSetterGeneratorOrNone::Generator
            | crate::GetterSetterGeneratorOrNone::None => {
                let id = func_ty.id;
                types.functions.insert(id, func_ty);
                let ty = types.register_type(Type::Function(id, Default::default()));
                environment.facts.events.push(Event::CreateObject {
                    prototype: crate::events::PrototypeArgument::Function(id),
                    referenced_in_scope_as: ty,
                });
                Property::Value(ty)
            }
        }
    }
}

pub trait SynthesizableFunction {
    fn is_declare(&self) -> bool;

    fn is_async(&self) -> bool;

    fn get_set_generator_or_none(&self) -> GetterSetterGeneratorOrNone;

    fn id(&self) -> FunctionId;

    /// **THIS FUNCTION IS EXPECTED TO PUT THE TYPE PARAMETERS INTO THE ENVIRONMENT WHILE SYNTHESIZING THEM**
    fn type_parameters<T: FSResolver>(
        &self,
        environment: &mut Environment,
        checking_data: &mut CheckingData<T>,
    ) -> Option<GenericTypeParameters>;

    /// Has to be the first parameter
    fn this_constraint<T: FSResolver>(
        &self,
        environment: &mut Environment,
        checking_data: &mut CheckingData<T>,
    ) -> Option<TypeId>;

    /// **THIS FUNCTION IS EXPECTED TO PUT THE PARAMETERS INTO THE ENVIRONMENT WHILE SYNTHESIZING THEM**
    fn parameters<T: FSResolver>(
        &self,
        environment: &mut Environment,
        checking_data: &mut CheckingData<T>,
    ) -> SynthesizedParameters;

    /// Returned type is extracted from events, thus doesn't expect anything in return
    fn body<T: FSResolver>(
        &self,
        environment: &mut Environment,
        checking_data: &mut CheckingData<T>,
    );

    fn return_type_annotation<T: FSResolver>(
        &self,
        environment: &mut Environment,
        checking_data: &mut CheckingData<T>,
    ) -> Option<(TypeId, Span)>;
}

struct ArrowFunction {
    is_async: bool,
}

struct Getter;
struct Setter;

enum Method {
    Getter,
    Setter,
    Generator { is_async: bool },
    Regular { is_async: bool },
}

struct StatementOrExpressionFunction {
    is_generator: bool,
    is_async: bool,
}

struct ClassConstructor {
    // events..?
    fields: (),
}

#[derive(Clone, Debug, Default, binary_serialize_derive::BinarySerializable)]
pub struct ClosedOverVariables(pub(crate) HashMap<VariableId, TypeId>);

#[derive(Clone, Copy, Debug, Default, binary_serialize_derive::BinarySerializable)]
pub enum ThisValue {
    Passed(TypeId),
    #[default]
    UseParent,
}

impl ThisValue {
    pub(crate) fn get(&self, environment: &mut Environment, types: &mut TypeStore) -> TypeId {
        match self {
            ThisValue::Passed(value) => *value,
            ThisValue::UseParent => environment.get_value_of_this(types),
        }
    }

    pub(crate) fn unwrap(&self) -> TypeId {
        match self {
            ThisValue::Passed(value) => *value,
            ThisValue::UseParent => panic!("Tried to get value of this"),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, binary_serialize_derive::BinarySerializable)]
pub struct ClosureId(pub(crate) u32);

pub trait ClosureChain {
    fn get_fact_from_closure<T, R>(&self, fact: &Facts, cb: T) -> Option<R>
    where
        T: Fn(ClosureId) -> Option<R>;
}

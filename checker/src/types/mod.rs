pub mod calling;
mod casts;
pub mod functions;
pub mod poly_types;
pub mod printing;
pub mod properties;
pub mod store;
pub mod subtyping;
mod terms;

use derive_debug_extras::DebugExtras;

pub(crate) use poly_types::specialization::*;

pub(crate) use casts::*;
use source_map::Span;
pub use store::TypeStore;
pub use terms::Constant;

use crate::{
    behavior::{
        functions::{ClosureId, ThisValue},
        operations::{CanonicalEqualityAndInequality, MathematicalAndBitwise, PureUnary},
    },
    events::RootReference,
    Environment, TruthyFalsy,
};

pub use self::functions::*;
use self::{
    poly_types::{generic_type_arguments::StructureGenericArguments, SeedingContext},
    subtyping::type_is_subtype,
};
use crate::FunctionId;

/// References [Type]
///
/// Not to be confused with [parser::TypeId]
///
/// TODO maybe u32 or u64
/// TODO maybe on environment rather than here
#[derive(PartialEq, Eq, Clone, Copy, DebugExtras, Hash)]
pub struct TypeId(pub(crate) u16);

// TODO ids as macro as to not do in [crate::RootEnvironment]
impl TypeId {
    /// Not to be confused with [TypeId::NEVER_TYPE]
    pub const ERROR_TYPE: Self = Self(0);
    pub const UNIMPLEMENTED_ERROR_TYPE: TypeId = TypeId::ERROR_TYPE;

    pub const NEVER_TYPE: Self = Self(1);

    pub const ANY_TYPE: Self = Self(2);

    pub const BOOLEAN_TYPE: Self = Self(3);
    pub const NUMBER_TYPE: Self = Self(4);
    pub const STRING_TYPE: Self = Self(5);

    pub const UNDEFINED_TYPE: Self = Self(6);
    pub const NULL_TYPE: Self = Self(7);

    /// This is the inner version
    pub const ARRAY_TYPE: Self = Self(8);

    /// Used for Array and Promise
    pub const T_TYPE: Self = Self(9);

    pub const OBJECT_TYPE: Self = Self(10);
    pub const FUNCTION_TYPE: Self = Self(11);
    pub const REGEXP_TYPE: Self = Self(12);
    pub const SYMBOL_TYPE: Self = Self(13);

    /// For more direct stuff and the rules
    pub const TRUE: Self = Self(14);
    pub const FALSE: Self = Self(15);
    pub const ZERO: Self = Self(16);
    pub const ONE: Self = Self(17);
    pub const NAN_TYPE: Self = Self(18);
    /// For arrays
    pub const LENGTH_AS_STRING: Self = Self(19);

    /// For this_arg for type constraints only
    pub const THIS_ARG: Self = Self(20);
    /// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/new.target
    pub const NEW_TARGET_ARG: Self = Self(21);

    pub const SYMBOL_TO_PRIMITIVE: Self = Self(22);

    // This exists in TS
    pub const HTML_ELEMENT_TAG_NAME_MAP: Self = Self(23);

    pub(crate) const INTERNAL_TYPE_COUNT: usize = 25;
}

#[derive(Clone, Debug, binary_serialize_derive::BinarySerializable)]
pub enum Type {
    /// For
    /// - `extends` interface part (for multiple it to is a `Type::And`)
    /// - Can be used for subtypes (aka N aliases number then more types on top)
    /// - **Does not imply .prototype = **
    AliasTo {
        to: TypeId,
        name: String,
        parameters: Option<Vec<TypeId>>,
    },
    And(TypeId, TypeId),
    Or(TypeId, TypeId),
    RootPolyType(PolyNature),
    /// Also a "Specialized constructor type"
    Constructor(Constructor),
    /// For number and other rooted types
    ///
    /// Although they all alias Object
    NamedRooted {
        name: String,
        parameters: Option<Vec<TypeId>>,
    },
    /// *Dependent equality types*
    Constant(crate::Constant),
    /// Represents known functions
    Function(FunctionId, ThisValue),

    /// From a type annotation or .d.ts WITHOUT body. e.g. don't know effects TODO...
    FunctionReference(FunctionId, ThisValue),

    /// Technically could be just a function but...
    Class(FunctionId),
    Object(ObjectNature), // TODO Proxy,
}

/// TODO difference between untyped and typed parameters and what about parameter based for any
#[derive(Clone, Debug, binary_serialize_derive::BinarySerializable)]
pub enum PolyNature {
    Parameter {
        fixed_to: TypeId,
    },
    Generic {
        name: String,
        /// This can be `Dynamic` for interface hoisting
        eager_fixed: TypeId,
    },
    Open(TypeId),
    /// For functions and for loops where something in the scope can mutate (so not constant)
    /// between runs.
    ParentScope {
        reference: RootReference,
        based_on: TypeId,
    },
    RecursiveFunction(FunctionId, TypeId),
    // Object
}

// TODO
pub fn is_primitive(ty: TypeId, types: &TypeStore) -> bool {
    if matches!(
        ty,
        TypeId::BOOLEAN_TYPE | TypeId::NUMBER_TYPE | TypeId::STRING_TYPE
    ) {
        return true;
    }
    return false;
}

#[derive(Copy, Clone, Debug, binary_serialize_derive::BinarySerializable)]
pub enum ObjectNature {
    /// Actual allocated object
    RealDeal,
    /// From `x: { a: number }` etc
    AnonymousTypeAnnotation,
}

impl Type {
    pub(crate) fn get_parameters(&self) -> Option<Vec<TypeId>> {
        if let Type::NamedRooted { parameters, .. } | Type::AliasTo { parameters, .. } = self {
            parameters.clone()
        } else {
            None
        }
    }

    /// TODO return is poly
    pub(crate) fn is_dependent(&self) -> bool {
        match self {
            // TODO
            Type::Constructor(Constructor::StructureGenerics(..)) => false,
            // Fine
            Type::Constructor(_) | Type::RootPolyType(_) => true,
            // TODO what about if left or right
            Type::And(_, _) | Type::Or(_, _) => false,
            // TODO what about if it aliases
            Type::AliasTo { .. } | Type::NamedRooted { .. } => {
                // TODO not sure
                false
            }
            Type::Constant(_)
            | Type::Function(..)
            | Type::FunctionReference(..)
            | Type::Class(..)
            | Type::Object(_) => false,
        }
    }
}

/// TODO work in progress types
#[derive(Clone, Debug, binary_serialize_derive::BinarySerializable)]
pub enum Constructor {
    // TODO separate add?
    BinaryOperator {
        lhs: TypeId,
        operator: MathematicalAndBitwise,
        rhs: TypeId,
    },
    CanonicalRelationOperator {
        lhs: TypeId,
        operator: CanonicalEqualityAndInequality,
        rhs: TypeId,
    },
    UnaryOperator {
        operator: PureUnary,
        operand: TypeId,
    },
    TypeOperator(TypeOperator),
    TypeRelationOperator(TypeRelationOperator),
    /// TODO constraint is res
    ConditionalResult {
        /// TODO this can only be poly types
        condition: TypeId,
        truthy_result: TypeId,
        else_result: TypeId,
        result_union: TypeId,
    },
    ///
    FunctionResult {
        on: TypeId,
        // TODO I don't think this is necessary, maybe for debugging. In such case should be an Rc to share with events
        with: Box<[SynthesizedArgument]>,
        result: TypeId,
    },
    Property {
        on: TypeId,
        under: TypeId,
    },
    /// Might not be best place but okay.
    StructureGenerics(StructureGenerics),
}

/// Curries arguments
#[derive(Clone, Debug, binary_serialize_derive::BinarySerializable)]
pub struct StructureGenerics {
    pub on: TypeId,
    pub arguments: StructureGenericArguments,
}

#[derive(Clone, Debug, binary_serialize_derive::BinarySerializable)]
pub enum TypeOperator {
    PrototypeOf(TypeId),
    PrimitiveTypeName(TypeId),
}

/// TODO instance of?
#[derive(Clone, Debug, binary_serialize_derive::BinarySerializable)]
pub enum TypeRelationOperator {
    Extends { ty: TypeId, extends: TypeId },
}

pub(crate) fn new_logical_or_type(lhs: TypeId, rhs: TypeId, types: &mut TypeStore) -> TypeId {
    types.new_conditional_type(lhs, lhs, rhs)
}

pub fn is_type_truthy_falsy(ty: TypeId, types: &TypeStore) -> TruthyFalsy {
    if ty == TypeId::TRUE || ty == TypeId::FALSE {
        TruthyFalsy::Decidable(ty == TypeId::TRUE)
    } else {
        let ty = types.get_type_by_id(ty);
        match ty {
            Type::AliasTo { .. }
            | Type::And(_, _)
            | Type::Or(_, _)
            | Type::RootPolyType(_)
            | Type::Constructor(_)
            | Type::NamedRooted { .. } => {
                // TODO some of these case are known
                TruthyFalsy::Unknown
            }
            Type::Function(..)
            | Type::FunctionReference(..)
            | Type::Class(..)
            | Type::Object(_) => TruthyFalsy::Decidable(true),
            Type::Constant(cst) => {
                // TODO strict casts
                TruthyFalsy::Decidable(cast_as_boolean(cst, false).unwrap())
            }
        }
    }
}

/// TODO add_property_restrictions via const generics
pub struct BasicEquality {
    pub add_property_restrictions: bool,
    pub position: Span,
}

/// For subtyping
pub trait SubtypeBehavior {
    fn set_type_argument(
        &mut self,
        parameter: TypeId,
        value: TypeId,
        environment: &mut Environment,
        types: &TypeStore,
    ) -> Result<(), NonEqualityReason>;

    fn add_property_restrictions(&self) -> bool;

    fn add_function_restriction(
        &mut self,
        environment: &mut Environment,
        function_id: FunctionId,
        function_type: FunctionType,
    );

    // TODO
    // object reference type needs to meet constraint
    // LHS is dependent + RHS argument
}

impl SubtypeBehavior for SeedingContext {
    /// Does not check thingy
    fn set_type_argument(
        &mut self,
        type_id: TypeId,
        value: TypeId,
        environment: &mut Environment,
        types: &TypeStore,
    ) -> Result<(), NonEqualityReason> {
        let restriction = self.type_arguments.get_restriction_for_id(type_id);

        // Check restriction from call site type argument
        if let Some((pos, restriction)) = restriction {
            if let SubTypeResult::IsNotSubType(reason) =
                type_is_subtype(restriction, value, None, self, environment, types)
            {
                return Err(NonEqualityReason::GenericRestrictionMismatch {
                    restriction,
                    reason: Box::new(reason),
                    pos,
                });
            }
        }

        self.type_arguments.set_id(type_id, value, types);

        Ok(())
    }

    fn add_property_restrictions(&self) -> bool {
        false
    }

    fn add_function_restriction(
        &mut self,
        _environment: &mut Environment,
        function_id: FunctionId,
        function_type: FunctionType,
    ) {
        self.locally_held_functions
            .insert(function_id, function_type);
    }
}

impl SubtypeBehavior for BasicEquality {
    fn set_type_argument(
        &mut self,
        parameter: TypeId,
        value: TypeId,
        environment: &mut Environment,
        types: &TypeStore,
    ) -> Result<(), NonEqualityReason> {
        Ok(())
    }

    fn add_property_restrictions(&self) -> bool {
        self.add_property_restrictions
    }

    fn add_function_restriction(
        &mut self,
        environment: &mut Environment,
        function_id: FunctionId,
        function_type: FunctionType,
    ) {
        let result = environment
            .deferred_function_constraints
            .insert(function_id, (function_type, self.position.clone()));

        debug_assert!(result.is_none());
    }
}

#[derive(Debug)]
pub enum SubTypeResult {
    IsSubType,
    IsNotSubType(NonEqualityReason),
}

// impl SubTypeResult {
// 	type Error = NonEqualityReason;
// }

// TODO implement `?` on SupertypeResult

// TODO maybe positions and extra information here
// SomeLiteralMismatch
// GenericParameterCollision
#[derive(Debug)]
pub enum NonEqualityReason {
    Mismatch,
    PropertiesInvalid {
        errors: Vec<(TypeId, PropertyError)>,
    },
    // For function call-site type arguments
    GenericRestrictionMismatch {
        restriction: TypeId,
        reason: Box<NonEqualityReason>,
        pos: Span,
    },
    TooStrict,
    /// TODO more information
    MissingParameter,
}

#[derive(Debug)]
pub enum PropertyError {
    Missing,
    Invalid {
        expected: TypeId,
        found: TypeId,
        mismatch: NonEqualityReason,
    },
}

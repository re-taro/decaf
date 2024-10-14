//! Contains wrappers for generic type arguments and their wrapping environments
//! TODO Some of these are a bit overkill and don't need wrapping objects **AND THEY BREAK FINALIZE THINGS REQUIRE CLONING**

use crate::{
    behavior::functions::{ClosureChain, ClosureId},
    context::facts::Facts,
    types::TypeStore,
    CheckingData, TypeId,
};

use map_vec::Map as SmallMap;
use source_map::Span;

use std::{fmt::Debug, iter::FromIterator};

use super::{GenericStructureTypeArgument, GenericStructureTypeArguments, ResolveGenerics};

// This is for `function x<T>(a: T): Array<T>`
impl ResolveGenerics for GenericStructureTypeArguments {
    fn resolve_generics<T: crate::FSResolver>(
        self,
        type_arguments: &FunctionTypeArguments,
        checking_data: &mut CheckingData<T>,
    ) -> Self {
        self.0
            .iter()
            .cloned()
            .map(|generic_type_argument_pair| {
                ResolveGenerics::resolve_generics(
                    generic_type_argument_pair,
                    type_arguments,
                    checking_data,
                )
            })
            .collect()
    }
}

impl FromIterator<GenericStructureTypeArgument> for GenericStructureTypeArguments {
    fn from_iter<I: IntoIterator<Item = GenericStructureTypeArgument>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

// impl CurriedGenericsValue {
// 	pub(crate) fn as_type(&self) -> TypeId {
// 		let (CurriedGenericsValue::Type(value) | CurriedGenericsValue::StructureGeneric(value)) =
// 			self;
// 		value
// 	}
// }

// #[derive(Debug, Clone)]
// pub enum CurriedGenericsValue {
// 	Type(TypeId),
// 	/// TODO this is different in that it can differ maybe... or is that closure values
// 	StructureGeneric(TypeId),
// }

// impl From<GenericStructureTypeArguments> for CurriedFunctionTypeArguments {
// 	fn from(args: GenericStructureTypeArguments) -> Self {
// 		Self(
// 			args.0
// 				.clone()
// 				.into_iter()
// 				.map(|GenericStructureTypeArgument { ref matching_id, ref ty, .. }| {
// 					(
// 						matching_id.clone().into(),
// 						todo!(),
// 						// CurriedGenericsValue::StructureGeneric(match ty.clone() {
// 						// 	super::GenericStructureArgumentValue::Type(ty) => ty,
// 						// 	super::GenericStructureArgumentValue::Unknown => todo!(),
// 						// }),
// 					)
// 				})
// 				.collect(),
// 		)
// 	}
// }

#[derive(Debug, Clone)]
pub(crate) struct FunctionTypeArgument {
    pub value: Option<TypeId>,
    /// Via <> at call site. Note that backing types are held separately
    pub restriction: Option<(Span, TypeId)>,
}

/// TODO working out environment thingy
#[derive(Debug)]
pub(crate) struct FunctionTypeArguments {
    pub structure_arguments: Option<StructureGenericArguments>,
    /// Might not be full
    pub local_arguments: SmallMap<TypeId, FunctionTypeArgument>,
    pub closure_id: Option<ClosureId>,
}

pub(crate) trait TypeArgumentStore {
    /// Gets the value, not the constraint
    fn get_structure_argument(&self, id: TypeId) -> Option<TypeId>;

    fn get_local_argument(&self, id: TypeId) -> Option<TypeId>;

    fn get_argument(&self, id: TypeId) -> Option<TypeId> {
        self.get_local_argument(id)
            .or_else(|| self.get_structure_argument(id))
    }

    fn get_structural_closures(&self) -> Option<Vec<ClosureId>>;

    fn into_structural_generic_arguments(&self) -> StructureGenericArguments;

    fn is_empty(&self) -> bool;
}

impl ClosureChain for FunctionTypeArguments {
    fn get_fact_from_closure<T, R>(&self, fact: &Facts, mut cb: T) -> Option<R>
    where
        T: Fn(ClosureId) -> Option<R>,
    {
        if let Some(ref closure_id) = self.closure_id {
            let res = cb(*closure_id);
            if res.is_some() {
                return res;
            }
        }
        if let Some(ref parent) = self.structure_arguments {
            for closure_id in parent.closures.iter() {
                let res = cb(*closure_id);
                if res.is_some() {
                    return res;
                }
            }
        }
        None
    }
}

impl TypeArgumentStore for FunctionTypeArguments {
    fn get_structure_argument(&self, id: TypeId) -> Option<TypeId> {
        self.structure_arguments
            .as_ref()
            .and_then(|args| args.get_structure_argument(id))
        // self.structure_arguments
        // 	.as_ref()
        // 	.and_then(|structure_arguments| structure_arguments.0.get(id).map(|v| v.as_type()))
        // 	.or_else(|| self.local_arguments.get(id).and_then(|v| v.value.as_ref()))
    }

    fn get_local_argument(&self, id: TypeId) -> Option<TypeId> {
        self.local_arguments.get(&id).and_then(|arg| arg.value)
    }

    fn get_structural_closures(&self) -> Option<Vec<ClosureId>> {
        None
    }

    fn into_structural_generic_arguments(&self) -> StructureGenericArguments {
        // self.structure_arguments.clone()
        match self.structure_arguments {
            Some(ref parent) => {
                let mut merged = parent.type_arguments.clone();
                merged.extend(
                    self.local_arguments
                        .iter()
                        .map(|(ty, arg)| (*ty, arg.value.unwrap())),
                );
                StructureGenericArguments {
                    type_arguments: merged,
                    closures: parent
                        .closures
                        .iter()
                        .cloned()
                        .chain(self.closure_id.into_iter())
                        .collect(),
                }
            }
            None => StructureGenericArguments {
                type_arguments: self
                    .local_arguments
                    .iter()
                    .map(|(ty, arg)| (*ty, arg.value.unwrap()))
                    .collect(),
                closures: self.closure_id.into_iter().collect(),
            },
        }
    }

    fn is_empty(&self) -> bool {
        self.closure_id.is_none() && self.local_arguments.len() == 0
    }
}

/// These are curried between structures
#[derive(Clone, Debug, binary_serialize_derive::BinarySerializable)]
pub struct StructureGenericArguments {
    pub type_arguments: map_vec::Map<TypeId, TypeId>,
    pub closures: Vec<ClosureId>,
}

impl TypeArgumentStore for StructureGenericArguments {
    fn get_structure_argument(&self, id: TypeId) -> Option<TypeId> {
        self.type_arguments.get(&id).copied()
    }

    fn get_local_argument(&self, id: TypeId) -> Option<TypeId> {
        None
    }

    fn get_structural_closures(&self) -> Option<Vec<ClosureId>> {
        Some(self.closures.clone())
    }

    fn into_structural_generic_arguments(&self) -> StructureGenericArguments {
        self.clone()
    }

    fn is_empty(&self) -> bool {
        self.closures.is_empty() && self.type_arguments.len() == 0
    }
}

impl FunctionTypeArguments {
    /// This is from <T>
    pub(crate) fn get_restriction_for_id(&self, id: TypeId) -> Option<(Span, TypeId)> {
        self.local_arguments
            .get(&id)
            .and_then(|arg| arg.restriction.clone())
        // self.structure_arguments
        // 	.as_ref()
        // 	.and_then(|structure_arguments| structure_arguments.0.get(&id).map(|v| v.as_type()))
        // 	.or_else(|| self.local_arguments.get(&id).map(|v| v.restriction))
    }

    /// TODO check restriction here!
    /// TODO remove `Environment`
    pub(crate) fn set_id(&mut self, on: TypeId, arg: TypeId, _ts: &TypeStore) {
        // crate::utils::notify!(
        // 	"Setting argument {:?} to {:?}",
        // 	_ts.debug_type(on),
        // 	_ts.debug_type(arg)
        // );

        match self.local_arguments.entry(on) {
            map_vec::map::Entry::Occupied(mut exists) => {
                if let Some(value) = exists.get().value {
                    todo!("check existing entry")
                } else {
                    exists.get_mut().value = Some(arg);
                }
            }
            map_vec::map::Entry::Vacant(vacant) => {
                vacant.insert(FunctionTypeArgument {
                    value: Some(arg),
                    // TODO::
                    restriction: None,
                });
            }
        }
    }

    pub(crate) fn set_this(&mut self, arg: TypeId) {
        self.local_arguments.insert(
            TypeId::THIS_ARG,
            FunctionTypeArgument {
                value: Some(arg),
                restriction: None,
            },
        );
    }
}

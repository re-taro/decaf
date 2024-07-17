use std::marker::PhantomData;

pub type EmptyCursorId = CursorId<()>;

impl EmptyCursorId {
	pub fn new(id: u8) -> Self {
		Self(id, PhantomData::default())
	}

	pub(crate) fn into_cursor<T>(self) -> CursorId<T> {
		CursorId(self.0, PhantomData::default())
	}
}

/// A cursor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorId<T>(pub u8, pub PhantomData<T>);

// Custom implementation used by the generator to interpolate nodes
#[cfg(feature = "self-rust-tokenize")]
impl<T> self_rust_tokenize::SelfRustTokenize for CursorId<T> {
	fn append_to_token_stream(
		&self,
		token_stream: &mut self_rust_tokenize::proc_macro2::TokenStream,
	) {
		use self_rust_tokenize::proc_macro2::{Ident, Span};
		let token = Ident::new(&format!("_cursor_{}", self.0), Span::call_site());
		token_stream.extend(self_rust_tokenize::quote!(parser::IntoAST::into_ast(#token)))
	}
}

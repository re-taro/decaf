#![no_main]

mod common {
	include!(concat!(env!("OUT_DIR"), "/common.rs")); // from build.rs
}

use boa_interner::ToInternedString;
use decaf_parser::{ASTNode, Module, ParseOutput, SourceId, ToStringSettingsAndData};
use libfuzzer_sys::{fuzz_target, Corpus};
use pretty_assertions::assert_eq;

fn do_fuzz(data: common::FuzzSource) -> Corpus {
	let input = data.source;

	let Ok(ParseOutput(module, state)) =
		Module::from_string(input.to_owned(), Default::default(), SourceId::NULL, None, Vec::new())
	else {
		return Corpus::Reject;
	};

	let output1 =
		module.to_string(&ToStringSettingsAndData(Default::default(), state.function_extractor));

	let ParseOutput(module, state) = Module::from_string(
		output1.to_owned(),
		Default::default(),
		SourceId::NULL,
		None,
		Vec::new(),
	)
	.expect(&(format!("\ninput: `{input}`\noutput1: `{output1}`\n\nThis parse should not error because it was just parsed above")));

	let output2 =
		module.to_string(&ToStringSettingsAndData(Default::default(), state.function_extractor));

	assert_eq!(output1, output2);

	Corpus::Keep
}

fuzz_target!(|data: common::FuzzSource| {
	do_fuzz(data);
});

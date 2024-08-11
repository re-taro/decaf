use decaf_parser::{ASTNode, JSXRoot, ParseOutput, SourceId, ToStringSettingsAndData};

fn main() {
    let source = "<Layout> <p>My page content, wrapped in a layout!</p> </Layout>";
    let ParseOutput(result, state) = JSXRoot::from_string(
        source.to_owned(),
        Default::default(),
        SourceId::NULL,
        None,
        Vec::new(),
    )
    .unwrap();

    println!(
        "{}",
        result.to_string(&ToStringSettingsAndData(
            Default::default(),
            state.function_extractor
        ))
    );
}

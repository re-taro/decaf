use decaf_ast_generator::stmt;

fn main() {
    let content = "World!";
    let my_stmt = stmt!(let my_element = <h1>Hello {#content}</h1>);
    println!(
        "{}",
        decaf_parser::ASTNode::to_string(&my_stmt, &Default::default())
    );
}

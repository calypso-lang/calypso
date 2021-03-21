fn main() {
    lalrpop::Configuration::new()
        .process_file("src/parser/grammar.lalrpop")
        .unwrap();
}

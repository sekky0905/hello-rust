enum ProgrammingLang {
    Rust,
    Go,
    TypeScript,
}

fn main () {
    let lang_rust = ProgrammingLang::Rust;
    print_lang(lang_rust);
    let lang_go = ProgrammingLang::Go;
    print_lang(lang_go);
    let lang_typescript = ProgrammingLang::TypeScript;
    print_lang(lang_typescript);
}

fn print_lang(lang: ProgrammingLang) {
    match lang {
        ProgrammingLang::Rust => println!("Rust"),
        ProgrammingLang::Go => println!("Go"),
        ProgrammingLang::TypeScript => println!("TypeScript"),
    }
}




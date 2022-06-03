#[derive(Debug)]
struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

fn build_scanner(source: String){
    Scanner {
        source,
        tokens = Vec::new()
    }
}



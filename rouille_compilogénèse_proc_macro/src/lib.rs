use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Mear" => "Err",
        "Ceart" => "Ok",
        "Sreang" => "String",
        "Faclair" => "HashMap",
        "Bun" => "Default",
        "Mearachd" => "Error",
        "Dòcha" => "Option",
        "Beagan" => "Some",
        "ChanEilSìon" => "None",
        "Buil" => "Result",
        "Fhèin" => "Self",
        "lèirig" => "println",
        "sguir" => "break",
        "neo_shionc" => "async",
        "feith" => "await",
        "dul" => "loop",
        "gluais" => "move",
        "cliath_bhogsa" => "crate",
        "còd_neo_ruigsinneach" => "unreachable_code",
        "mar" => "as",
        "cunbhalachan" => "const",
        "trèithe" => "trait",
        "gàbhach" => "unsafe",
        "ann_an" => "in",
        "bho" => "from",
        "dinimigeach" => "dyn",
        "tuainig" => "unwrap",
        "bun" => "default",
        "en_réf" => "mar_reif",
        "ia" => "io",
        "a_muigh" => "extern",
        "fallsa" => "false",
        "foincsean" => "fn",
        "pàrant" => "super",
        "cuir" => "insert",
        "leugh" => "get",
        "leig" => "allow",
        "a_ghalla" | "ar_son_diabhal" | "gabh_giorag" => "panic",
        "co_phàirt" => "mod",
        "caochlaideach" => "mut",
        "ùr" => "new",
        "far_a" => "where",
        "airson" => "for",
        "gabh_no_cuir_le" => "get_or_insert_with",
        "priomhail" => "main",
        "poblach" => "pub",
        "till" => "return",
        "toirt_gu_buil" => "impl",
        "reif" => "ref",
        "maidsich" => "match",
        "ma" => "if",
        "air_neo" => "else",
        "fhèin" => "self",
        "biodh" => "let",
        "stadaigeach" => "static",
        "structar" => "struct",
        "sùilich" => "expect",
        "nuair" => "while",
        "dèan_feum_de" => "use",
        "gu" => "into",
        "fìor" => "true",
        "àireamh" => "enum",
        "Baid" => "Group",
        "Aithnichear" => "Ident",
        "SruthTòcain" => "TokenStream",
        "CraobhTòcain" => "TokenTree",
        "gu_sreang" => "to_string",
        "mar_sreang" => "as_str",
        "rainse" => "span",
        "Vector" => "Vec",
        "Sruth" => "stream",
        "ut" => "push",
        "sìn" => "extend",
        "comharra_crìche" => "delimiter",
        "Pungadh" => "Punct",
        "Litearail" => "Literal",
        "macro_modha" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn meirg(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}

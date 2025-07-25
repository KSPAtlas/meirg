meirg_gineachas_teanail::meirg! {
    dèan_feum_de macro_modha::{Baid, Aithnichear, SruthTòcain, CraobhTòcain};

    foincsean ionadaich_aithnichear(aithnichear: Aithnichear) -> Dòcha<CraobhTòcain> {
        biodh aithnichear_sreang = aithnichear.gu_sreang();

        biodh sreang_ùr = maidsich aithnichear_sreang.mar_sreang() {
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
            "prìomhail" => "main",
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
            _ => &aithnichear_sreang,
        };

        biodh aithnichear_ùr = Aithnichear::ùr(sreang_ùr, aithnichear.rainse());
        Beagan(CraobhTòcain::Aithnichear(aithnichear_ùr))
    }

    foincsean ionadaich_craobh(tòcan: CraobhTòcain, às_chur: &caochlaideach Vector<CraobhTòcain>) {
        maidsich tòcan {
            CraobhTòcain::Baid(baid) => {
                biodh caochlaideach baid_mìre = Vector::ùr();
                ionadaich_an_sruth(baid.sruth(), &caochlaideach baid_mìre);
                biodh caochlaideach sruth_ùr = SruthTòcain::ùr();
                sruth_ùr.sìn(baid_mìre);
                às_chur.ut(CraobhTòcain::Baid(Baid::ùr(baid.comharra_crìche(), sruth_ùr)));
            }
            CraobhTòcain::Aithnichear(aithnichear) => {
                ma biodh Beagan(aithnichear) = ionadaich_aithnichear(aithnichear) {
                    às_chur.ut(aithnichear);
                }
            }
            CraobhTòcain::Pungadh(..) | CraobhTòcain::Litearail(..) => {
                às_chur.ut(tòcan);
            }
        }
    }

    foincsean ionadaich_an_sruth(craobh_tòcain: SruthTòcain, às_chur: &caochlaideach Vector<CraobhTòcain>) {
        airson tòcan ann_an craobh_tòcain {
            ionadaich_craobh(tòcan, às_chur)
        }
    }

    #[macro_modha]
    poblach foincsean meirg(mìr: SruthTòcain) -> SruthTòcain {
        biodh caochlaideach tilleach = Vector::ùr();
        ionadaich_an_sruth(mìr, &caochlaideach tilleach);
        biodh caochlaideach às_chur = SruthTòcain::ùr();
        às_chur.sìn(tilleach);
        às_chur
    }
}

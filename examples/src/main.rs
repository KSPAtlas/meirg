meirg::meirg! {
    a_muigh cliath_bhogsa meirg;

   dèan_feum_de std::collections::faclair mar Facl;

    trèithe IuchairLuach {
        foincsean sgrìobh(&fhèin, iuchair: Sreang, luach: Sreang);
        foincsean leugh(&fhèin, iuchair: Sreang) -> Dòcha<&Sreang>;
    }

    stadaigeach caochlaideach FACLAIR: Dòcha<Facl<Sreang, Sreang>> = ChanEilSìon;

    structar Riochdail;

    toirt_gu_buil IuchairLuach airson Riochdail {
        foincsean sgrìobh(&fhèin, iuchair: Sreang, luach: Sreang) {
            biodh facl = gàbhach {
                FACLAIR.gabh_no_cuir_le(Bun::bun)
            };
            facl.cuir(iuchair, luach);
        }
        foincsean leugh(&fhèin, iuchair: Sreang) -> Buil<Dòcha<&Sreang>, Sreang> {
            ma biodh Beagan(facl) = gàbhach { FACLAIR.mar_reif() } {
                Ceart(facl.leugh(&iuchair))
            } air_neo {
                Mear("faigh an faclair".gu())
            }
        }
    } 

    poblach (cliath_bhogsa) foincsean dòcha(i: u32) -> Dòcha<Buil<u32, Sreang>> {
        ma i % 2 == 1 {
            ma i == 42 {
                Beagan(Mear(Sreang::bho("a ghalla")))
            } air_neo {
                Beagan(Ceart(33))
            }
        } air_neo {
            ChanEilSìon
        }
    }

    neo_shionc foincsean eisimpleir() {
    }

    neo_shionc foincsean eisimpleir2() {
        eisimpleir().feith;
    }

    foincsean prìomhail() {
        biodh caochlaideach x = 31;

        maidsich x {
            42 => {
                lèirig!("uisge-beatha")
            }
            _ => lèirig!("sin agadsa dhut")
        }

        airson i ann_an 0..10 {
            biodh luach = dul {
                sguir i;
            };

            nuair x < luach {
                x += 1;
            }

            x = ma biodh Beagan(buil) = dòcha(i) {
                buil.tuainig()
            } air_neo {
                12
            };
        }

        //dàrnach();
    }

    #[leig(còd_neo_ruigsinneach)]
    foincsean dàrnach() {
        a_ghalla!("ò mo chreach"); // for the true Scottish Gaelic experience
        ar_son_diabhal!("ó mo thrua"); // for friends speaking Irish
        gabh_giorag!("abab"); // in SFW contexts
    }
}

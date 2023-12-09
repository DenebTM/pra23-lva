pub mod chaotic_iter {
    use crate::{
        analysis::{lv_entry, lv_exit, LVAnalysis},
        program::Program,
    };

    pub fn run<'a>(program: &'a Program<'a>) -> LVAnalysis {
        let mut lva: LVAnalysis = LVAnalysis::new(program.len);

        loop {
            let lva_next = LVAnalysis {
                exit: lv_exit(program, &lva.entry),
                entry: lv_entry(program, &lva.exit),
            };

            if lva_next == lva {
                break;
            }

            lva = lva_next;
        }

        lva
    }
}

pub mod mfp {
    use crate::{
        analysis::{lv_entry, lv_entry_at, LVAnalysis, LVExit},
        expression::{Label, Variable},
        program::Program,
    };
    use std::collections::HashSet;

    type Lattice = HashSet<Variable>;
    type Analysis = LVExit;

    pub fn run<'a>(program: &'a Program<'a>) -> LVAnalysis {
        let bottom: Lattice = [].into();
        let ext_lab: HashSet<Label> = [program.init_label()].into();
        let ext_val: Lattice = [].into();
        let flow = program.flow_r();
        let f_l = |p: &Program, a: &LVExit| lv_entry(p, a);
        let f_l_at = |p: &Program, a: &LVExit, l: Label| lv_entry_at(p, a, l);

        // step 1: initialize
        let mut work_list: Vec<(Label, Label)> = flow.iter().cloned().collect();
        let mut ana: Analysis = (1..=program.len)
            .map(|label| {
                (
                    label,
                    if ext_lab.contains(&label) {
                        ext_val.clone()
                    } else {
                        bottom.clone()
                    },
                )
            })
            .collect();

        // step 2: iterate
        while work_list.len() > 0 {
            let (l, l_p) = work_list.remove(0);

            let (a, b) = (f_l_at(program, &ana, l), &ana[&l_p]);
            if !a.is_subset(&b) {
                ana.insert(l_p, [a, b.clone()].iter().flatten().cloned().collect());

                flow.iter()
                    .filter(|(l_p_2, _)| &l_p == l_p_2)
                    .for_each(|f| {
                        work_list.insert(0, f.clone());
                    });
            }
        }

        // step 3: present
        LVAnalysis {
            exit: ana.clone(),
            entry: f_l(program, &ana),
        }
    }
}

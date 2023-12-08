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

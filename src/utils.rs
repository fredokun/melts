
#[derive(Debug)]
struct SourcePos {
    start_line: u32,
    start_col: u32,
    end_line: u32,
    end_col: u32
}

fn mk_pos(sl:u32, sc:u32, el:u32, ec:u32) -> SourcePos {
    SourcePos { start_line: sl, start_col: sc
                , end_line: el, end_col: ec }
}

impl SourcePos {

    fn show_lts(&self) -> String {
        format!("@pos({}:{}->{}:{})"
                , self.start_line
                , self.start_col
                , self.end_line
                , self.end_col)
    }

}


#[cfg(test)]
mod tests {

    use utils::*;

     #[test]
    fn test_source_pos_debug() {
        let spos = SourcePos { start_line: 0,
                               start_col: 0,
                               end_line: 0,
                               end_col: 0 };
        println!("{:?}", spos);
        assert_eq!(format!("{:?}", spos),
                   "SourcePos { start_line: 0, start_col: 0, end_line: 0, end_col: 0 }");
        assert_eq!(spos.show_lts(), "@pos(0:0->0:0)");
    }

    #[test]
    fn test_source_pos_mk() {
        let spos1 = SourcePos { start_line: 1,
                                start_col: 2,
                                end_line: 3,
                                end_col: 4 };

        let spos2 = mk_pos(1, 2, 3, 4);

        assert_eq!(spos1.start_line, spos2.start_line);
        assert_eq!(spos1.start_col, spos2.start_col);
        assert_eq!(spos1.end_line, spos2.end_line);
        assert_eq!(spos1.end_col, spos2.end_col);
    }

}

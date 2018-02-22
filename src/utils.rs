
#[derive(Debug)]
struct SourcePos {
    start_line: u32,
    end_line: u32,
    start_col: u32,
    end_col: u32
}



#[cfg(test)]
mod tests {

    use utils::SourcePos;

     #[test]
    fn test_source_pos_debug() {
        let spos = SourcePos { start_line: 0,
                               start_col: 0,
                               end_line: 0,
                               end_col: 0 };
        println!("{:?}", spos);
        assert_eq!(format!("{:?}", spos),
                   "SourcePos { start_line: 0, end_line: 0, start_col: 0, end_col: 0 }");;
    }

}

use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Debug)]
pub enum Line<'a> {
    CmdCd(&'a str),
    CmdLs,
    Dir(&'a str),
    File(u64, &'a str),
    Unknown(&'a str),
}

mod parse {
    use crate::Line;
    use nom;
    use nom::branch;
    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, not_line_ending, space1, u64};
    use nom::combinator;
    use nom::multi;
    use nom::sequence;
    use nom::IResult;

    fn cmd_cd(i: &str) -> IResult<&str, Line> {
        sequence::preceded(tag("$ cd "), combinator::map(not_line_ending, Line::CmdCd))(i)
    }

    fn cmd_ls(i: &str) -> IResult<&str, Line> {
        combinator::map(tag("$ ls"), |_| Line::CmdLs)(i)
    }

    fn dir(i: &str) -> IResult<&str, Line> {
        sequence::preceded(tag("dir "), combinator::map(not_line_ending, Line::Dir))(i)
    }

    fn file(i: &str) -> IResult<&str, Line> {
        combinator::map(
            sequence::separated_pair(u64, space1, not_line_ending),
            |(a, b)| Line::File(a, b),
        )(i)
    }

    fn unknown(i: &str) -> IResult<&str, Line> {
        combinator::map(not_line_ending, Line::Unknown)(i)
    }

    pub fn input(i: &str) -> IResult<&str, Vec<Line>> {
        multi::separated_list0(
            line_ending,
            branch::alt((cmd_cd, cmd_ls, dir, file, unknown)),
        )(i)
    }
}

fn mkkey(elems: &[&str]) -> String {
    elems.join("/")
}

fn main() {
    let input = fs::read_to_string("input/07.txt").unwrap();
    let lines = parse::input(&input).unwrap().1;
    let mut current_path: Vec<&str> = vec![];
    let mut sizes: HashMap<String, u64> = HashMap::new();
    for line in lines {
        match line {
            Line::CmdCd("..") => {
                current_path.pop();
            }
            Line::CmdCd(path) => {
                current_path.push(path);
                let key = mkkey(&current_path);
                if !sizes.contains_key(&key) {
                    sizes.insert(key, 0);
                }
            }
            Line::File(size, _) => {
                for i in 1..(current_path.len() + 1) {
                    *(sizes.get_mut(&mkkey(&current_path[0..i])).unwrap()) += size;
                }
            }
            _ => {}
        }
    }
    let mut total = 0;
    for (_, v) in sizes.iter() {
        if v <= &100000 {
            total += v
        }
    }
    println!("{:?}", total);
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> (usize, usize) {
    let mut s = input.lines().next().unwrap().split('-');
    (
        s.next().unwrap().parse::<usize>().unwrap(),
        s.next().unwrap().parse::<usize>().unwrap(),
    )
}

#[aoc(day4, part1)]
pub fn part1(input: &(usize, usize)) -> usize {
    let mut cnt = 0;
    for a in 1..=9 {
        for b in a..=9 {
            for c in b..=9 {
                for d in c..=9 {
                    for e in d..=9 {
                        for f in e..=9 {
                            let num = a * 100000 + b * 10000 + c * 1000 + d * 100 + e * 10 + f;
                            if num >= input.0
                                && num <= input.1
                                && (a == b || b == c || c == d || d == e || e == f)
                            {
                                cnt += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    cnt
}

#[aoc(day4, part2)]
pub fn part2(input: &(usize, usize)) -> usize {
    let mut cnt = 0;
    for a in 1..=9 {
        for b in a..=9 {
            for c in b..=9 {
                for d in c..=9 {
                    for e in d..=9 {
                        for f in e..=9 {
                            let num = a * 100000 + b * 10000 + c * 1000 + d * 100 + e * 10 + f;
                            if num >= input.0
                                && num <= input.1
                                && ((a == b && b != c)
                                    || (b == c && a != b && c != d)
                                    || (c == d && b != c && d != e)
                                    || (d == e && c != d && e != f)
                                    || (e == f && d != e))
                            {
                                cnt += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    cnt
}

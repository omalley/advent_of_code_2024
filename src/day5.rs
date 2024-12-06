use itertools::Itertools;
use smallvec::{SmallVec, ToSmallVec};

pub type RuleId = u16;

fn parse_int(s: &str) -> Result<RuleId, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

#[derive(Debug,Eq,Ord,PartialOrd,PartialEq)]
struct Rule {
  previous: RuleId,
  following: RuleId,
}

impl Rule {
  fn from_str(line: &str) -> Result<Rule, String> {
    let (previous_str, following_str) = line.split_once("|")
        .ok_or("missing divider")?;
    Ok(Rule{previous: parse_int(previous_str)?,
      following: parse_int(following_str)?})
  }
}

#[derive(Debug)]
pub struct RuleGroup {
  previous: RuleId,
  following_list: SmallVec<[RuleId; 32]>,
}

pub type Printing = SmallVec<[RuleId; 32]>;

fn parse_printing(line: &str) -> Result<Printing, String> {
  line.split(",").map(parse_int).try_collect()
}

#[derive(Debug)]
pub struct Input {
  rules: Vec<RuleGroup>,
  printings: Vec<Printing>,
  max_id: RuleId,
}

pub fn generator(input: &str) -> Input {
  let mut reading_rules = true;
  let mut simple_rules = Vec::new();
  let mut printings = Vec::new();
  for line in input.lines() {
    if line.is_empty() {
      reading_rules = false;
    } else if reading_rules {
      simple_rules.push(Rule::from_str(line).expect("Can't parse rule"));
    } else {
      printings.push(parse_printing(line).expect("Can't parse printing"));
    }
  }
  simple_rules.sort_unstable();
  let mut rules = Vec::new();
  let mut max_id = 0;
  for (previous, chunk) in &simple_rules.into_iter().chunk_by(|r| r.previous) {
    let following_list: SmallVec<[RuleId; 32]> = chunk.map(|r| r.following).collect();
    max_id = max_id.max(*following_list.iter().max().unwrap_or(&0));
    max_id = max_id.max(previous);
    rules.push(RuleGroup{previous, following_list})
  }
  Input{rules, printings, max_id}
}

fn find_rule(rules: &[RuleGroup], rule_id: RuleId) -> Option<&RuleGroup> {
  rules.binary_search_by(|probe| probe.previous.cmp(&rule_id))
      .map(|id| &rules[id]).ok()
}

fn is_order_correct(rules: &[RuleGroup], printing: &[RuleId], pad: &mut [bool]) -> bool {
  let mut result = true;
  'page: for page in printing {
    pad[*page as usize] = true;
    if let Some(group) = find_rule(rules, *page) {
      for follow in &group.following_list {
        if pad[*follow as usize] {
          result = false;
          break 'page;
        }
      }
    }
  }
  for page in printing {
    pad[*page as usize] = false;
  }
  result
}

fn find_middle(printing: &[RuleId]) -> RuleId {
  printing[printing.len() / 2]
}

pub fn part1(input: &Input) -> u64 {
  let mut pad = vec![false; input.max_id as usize + 1];
  input.printings.iter()
      .filter(|&pr| is_order_correct(&input.rules, pr, &mut pad))
      .map(|pr| find_middle(pr) as u64)
      .sum()
}

/// Find the index of the oldest violation of the current rule.
fn find_violation(rule: &RuleGroup, pad: &[Option<usize>]) -> Option<usize> {
  rule.following_list.iter().filter_map(|id| pad[*id as usize]).min()
}

fn fix_printing(rules: &[RuleGroup], printing: &[RuleId],
                pad: &mut [Option<usize>]) -> Option<Printing> {
  let mut was_broken = false;
  let mut fix = printing.to_smallvec();
  let mut i = 0;
  while i < fix.len() {
    let page = fix[i];
    // Mark the current page as done
    pad[page as usize] = Some(i);
    if let Some(rule) = find_rule(rules, page) {
      if let Some(violation) = find_violation(rule, pad) {
        was_broken = true;
        // clear the pad for the parts we need to recheck
        for j in violation+1..i {
          pad[fix[j] as usize] = None;
        }
        // move element i in front of the violation
        fix[violation..(i+1)].rotate_right(1);
        // Set the pad to mark the new locations
        pad[fix[violation] as usize] = Some(violation);
        pad[fix[violation+1] as usize] = Some(violation+1);
        i = violation;
      }
    }
    i += 1;
  }
  for page in printing {
    pad[*page as usize] = None;
  }
  if was_broken {
    Some(fix)
  } else {
    None
  }
}

pub fn part2(input: &Input) -> u64 {
  let mut pad = vec![None; input.max_id as usize + 1];
  input.printings.iter().filter_map(|pr| fix_printing(&input.rules, pr, &mut pad))
      .map(|pr| find_middle(&pr) as u64).sum()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(143, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(123, part2(&data));
  }
}

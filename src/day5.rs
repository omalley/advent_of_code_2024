use itertools::Itertools;
use smallvec::{SmallVec, ToSmallVec};

pub type PageId = u16;

fn parse_int(s: &str) -> Result<PageId, String> {
  s.parse().map_err(|_| format!("Can't parse integer - '{s}'"))
}

/// A single rule stating which page comes before another.
#[derive(Debug,Eq,Ord,PartialOrd,PartialEq)]
struct Rule {
  previous: PageId,
  following: PageId,
}

impl Rule {
  fn from_str(line: &str) -> Result<Rule, String> {
    let (previous_str, following_str) = line.split_once("|")
        .ok_or("missing divider")?;
    Ok(Rule{previous: parse_int(previous_str)?,
      following: parse_int(following_str)?})
  }
}

/// Grouping the rules by their previous page.
#[derive(Debug)]
pub struct RuleGroup {
  previous: PageId,
  following_list: PageList,
}

pub type PageList = SmallVec<[PageId; 32]>;

fn parse_printing(line: &str) -> Result<PageList, String> {
  line.split(",").map(parse_int).try_collect()
}

#[derive(Debug)]
pub struct Input {
  rules: Vec<RuleGroup>,
  printings: Vec<PageList>,
  max_id: PageId,
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
  // Sort the rules and group them together to form the rule groups.
  simple_rules.sort_unstable();
  let mut rules = Vec::new();
  let mut max_id = 0;
  for (previous, chunk) in &simple_rules.into_iter()
      .chunk_by(|r| r.previous) {
    let following_list: PageList = chunk.map(|r| r.following).collect();
    max_id = max_id.max(*following_list.iter().max().unwrap_or(&0));
    max_id = max_id.max(previous);
    rules.push(RuleGroup{previous, following_list})
  }
  Input{rules, printings, max_id}
}

/// Look up which RuleGroup applies.
fn find_rule(rules: &[RuleGroup], page: PageId) -> Option<&RuleGroup> {
  rules.binary_search_by(|probe| probe.previous.cmp(&page))
      .map(|id| &rules[id]).ok()
}

/// Is the order of the pages correct?
/// The pad is a vector large enough to index by any of the PageId and must be
/// false before and after the function call.
fn is_order_correct(rules: &[RuleGroup], printing: &[PageId], pad: &mut [bool]) -> bool {
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

/// Find the middle page by index.
fn find_middle(printing: &[PageId]) -> PageId {
  printing[printing.len() / 2]
}

pub fn part1(input: &Input) -> u64 {
  // Allocate a reusable scratch pag to record which pages we've processed.
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

/// If a given printing breaks the rules, fix the order of pages so that the
/// rules are satisfied. This must be a stable sort.
/// The pad is a scratch pad that must be large enough for any of the PageId and
/// must be None before and after the call.
fn fix_printing(rules: &[RuleGroup], printing: &[PageId],
                pad: &mut [Option<usize>]) -> Option<PageList> {
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

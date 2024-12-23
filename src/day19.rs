use std::str;
use trie_rs::Trie;
use trie_rs::inc_search::Answer;

pub struct Input {
  words: Trie<u8>,
  lines: Vec<String>,
}

pub fn generator(input: &str) -> Input {
  let (words, patterns) = input.split_once("\n\n")
      .expect("Can't split input");
  let words = Trie::from_iter(words.split(',').map(|w| w.trim()));
  let lines = patterns.lines().map(|line| line.to_owned()).collect();
  Input{words, lines}
}

fn match_line(words: &Trie<u8>, line: &[u8]) -> bool {
  let mut done = vec![false; line.len()];
  let mut search = words.inc_search();
  let mut backtrace = vec![0];
  while let Some(posn) = backtrace.pop() {
    if posn >= line.len() {
      return true;
    }
    if done[posn] {
      continue;
    }
    done[posn] = true;
    search.reset();
    for (i, ch) in line[posn..].iter().enumerate() {
      let res = search.query(ch);
      match res {
        None => { break; }
        Some(Answer::Prefix) => { },
        Some(Answer::Match) => {
          if posn + i + 1 == line.len() {
            return true;
          } else {
            search.reset();
          }
        },
        Some(Answer::PrefixAndMatch) => {
          backtrace.push(posn + i + 1);
        }
      }
    }
  }
  false
}

pub fn part1(input: &Input) -> usize {
  input.lines.iter().filter(|line| match_line(&input.words, line.as_bytes())).count()
}

fn count_patterns(words: &Trie<u8>, line: &[u8], cache: &mut Vec<Option<usize>>) -> usize {
  if let Some(result) = cache[line.len()] {
    return result;
  }
  let mut search = words.inc_search();
  let mut result = 0;
  for (i, ch) in line.iter().enumerate() {
    match search.query(ch) {
      None => { break; }
      Some(Answer::Prefix) => { },
      Some(Answer::Match) => {
        if i + 1 == line.len() {
          result += 1;
        } else {
          search.reset();
        }
      },
      Some(Answer::PrefixAndMatch) => {
        if i + 1 == line.len() {
          result += 1;
        } else {
          result += count_patterns(words, &line[i+1..], cache);
        }
      }
    }
  }
  cache[line.len()] = Some(result);
  result
}

pub fn part2(input: &Input) -> usize {
  input.lines.iter().map(|line| {
    let mut cache = vec![None; line.len() + 1];
    count_patterns(&input.words, line.as_bytes(), &mut cache)
  }).sum()
}

#[cfg(test)]
mod tests {
  use super::{generator, part1, part2};

  const INPUT: &str =
"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

  #[test]
  fn test_part1() {
    let data = generator(INPUT);
    assert_eq!(6, part1(&data));
  }

  #[test]
  fn test_part2() {
    let data = generator(INPUT);
    assert_eq!(16, part2(&data));
  }
}

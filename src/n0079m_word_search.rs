// https://leetcode.com/problems
pub struct Solution {}

// START SUBMISSION CODE

#[derive(Clone)]
struct Cursor {
    i: usize,
    j: usize,
    c: char
}

#[derive(Clone)]
struct Snake {
    body: Vec<Cursor>
}

struct Zoo {
    snakes: Vec<Snake>,
    target: Vec<char>,
    field: Vec<Vec<char>>
}

impl Zoo {
    fn new(field: Vec<Vec<char>>, target: Vec<char>) -> Self {
        Zoo {
            snakes: Vec::new(),
            target,
            field
        }
    }

    fn snake_if_candidate(&mut self, c: char, i: usize, j: usize) {
        if self.target[0] == c {
            self.snakes.push(Snake::new(c, i, j))
        }
    }

    fn determine_candidates(&mut self) {
        for i in 0..self.field.len() {
            for j in 0..self.field[0].len() {
                self.snake_if_candidate(self.field[i][j], i, j)
            }
        }
    }

    fn simulate(mut self) -> bool {
        self.determine_candidates();
        loop {
            if let Some(snake) = self.snakes.pop() {
                if snake.body.len() == self.target.len() {
                    break true
                }
                let mut new_snakes = snake.determine_children(&self);
                self.snakes.append(&mut new_snakes)
            } else {
                break false
            }
        }
    }
}

impl Snake {
    fn new(c: char, i: usize, j:usize) -> Self {
        let mut body: Vec<Cursor> = Vec::new();
        let cursor: Cursor = Cursor::new(c, i, j);
        body.push(cursor);

        Snake {
            body
        }
    }

    fn grow_child(&self, cursor: Cursor) -> Snake {
        let mut new = self.clone();
        new.body.push(cursor);
        new
    }

    fn determine_children(self, zoo: &Zoo) -> Vec<Snake> {
        let target = zoo.target[self.body.len()];
        let mut new_snakes = Vec::new();

        let cursors = self.determine_unused_next_cursors(zoo);
        for cursor in cursors {
            if cursor.c == target {
                new_snakes.push(self.grow_child(cursor))
            }
        }
        
        new_snakes
    }

    fn determine_unused_next_cursors(&self, zoo: &Zoo) -> Vec<Cursor> {
        let mut cursors = Vec::new();
        let i = self.body.last().unwrap().i;
        let j = self.body.last().unwrap().j;
        let mut top = true;
        let mut bottom = true;
        let mut left = true;
        let mut right = true;

        for cursor in &self.body {
            if cursor.i == i && cursor.j + 1 == j {
                left = false
            }
            if cursor.i == i && cursor.j == j + 1 {
                right = false
            }
            if cursor.i + 1 == i && cursor.j == j {
                top = false
            }
            if cursor.i == i + 1 && cursor.j == j {
                bottom = false
            }
        }

        if top && i > 0 {
            cursors.push(Cursor::new(zoo.field[i - 1][j], i - 1, j))
        }  
        if bottom && i < zoo.field.len() - 1 {
            cursors.push(Cursor::new(zoo.field[i + 1][j], i + 1, j))
        }
        if left && j > 0 {
            cursors.push(Cursor::new(zoo.field[i][j - 1], i, j - 1))
        }
        if right && j < zoo.field[0].len() - 1 {
            cursors.push(Cursor::new(zoo.field[i][j + 1], i, j + 1))
        }

        cursors
    }


}

impl Cursor {
    fn new(c: char, i: usize, j: usize) -> Self {
        Cursor {
            c,
            i,
            j
        }
    }
}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word_chars: Vec<char> = word.chars().collect();
        let zoo = Zoo::new(board, word_chars);
        zoo.simulate()
    }
}
 
// END SUBMISSION CODE
 
 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_1() {
         let board = vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']];
         let word = "ABCCED".to_string();
         assert_eq!(Solution::exist(board, word), true);
     }
 }
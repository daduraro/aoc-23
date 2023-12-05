advent_of_code::solution!(3);

#[derive(Debug)]
enum Cell {
    Empty,
    Symbol(char),
    Digit(u8),
}

#[derive(Debug)]
struct Map {
    cells: Vec<Cell>,
    width: i32,
    height: i32,
}

#[derive(Debug)]
struct MapNumbers<'a> {
    map: &'a Map,
    curr_x: i32,
    curr_y: i32,
}

#[derive(Debug)]
struct MapSymbols<'a> {
    map: &'a Map,
    curr_x: i32,
    curr_y: i32,
}

#[derive(Debug)]
struct MapWindow {
    y: i32,
    x: i32,
    width: i32,
}

trait Neighbouring {
    fn is_neighbour(&self, x: i32, y: i32) -> bool;
}

impl Neighbouring for MapWindow {
    fn is_neighbour(&self, x: i32, y: i32) -> bool {
        if y == self.y {
            x == self.x - 1 || x == self.x + self.width
        }
        else if y == self.y - 1  || y == self.y + 1 {
            x >= self.x - 1 && x <= self.x + self.width
        }
        else {
            false
        }
    }
}

trait Mapable {
    fn get(&self, x: i32, y: i32) -> Option<&Cell>;
    fn numbers(& self) -> MapNumbers;
    fn neighbours(& self, window: &MapWindow) -> Vec<(i32, i32)>;
    fn symbols(& self) -> MapSymbols;
}

impl Mapable for Map {
    fn get(&self, x: i32, y: i32) -> Option<&Cell> {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            return None;
        }
        self.cells.get((y * self.width + x) as usize)
    }

    fn numbers(& self) -> MapNumbers {
        MapNumbers {
            map: self,
            curr_x: 0,
            curr_y: 0,
        }
    }

    fn symbols(& self) -> MapSymbols {
        MapSymbols {
            map: self,
            curr_x: 0,
            curr_y: 0,
        }
    }

    fn neighbours(& self, window: &MapWindow) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        // top border
        if window.y > 0 {
            for x in (window.x-1)..(window.x + window.width + 1) {
                if let Some(Cell::Symbol(_)) = self.get(x, window.y - 1) {
                    result.push((x, window.y - 1));
                }
            }
        }

        // bottom border
        if window.y + 1 < self.height {
            for x in (window.x-1)..(window.x + window.width + 1) {
                if let Some(Cell::Symbol(_)) = self.get(x, window.y + 1) {
                    result.push((x, window.y + 1));
                }
            }
        }

        // left cell
        if window.x > 0 {
            if let Some(Cell::Symbol(_)) = self.get(window.x - 1, window.y) {
                result.push((window.x - 1, window.y));
            }
        }

        // right cell
        if window.x + window.width < self.width {
            if let Some(Cell::Symbol(_)) = self.get(window.x + window.width, window.y) {
                result.push((window.x + window.width, window.y));
            }
        }
        result
    }
}

impl Iterator for MapNumbers<'_> {
    type Item = (u32, MapWindow);

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = 0;
        let mut curr_x = self.curr_x;
        let mut curr_y = self.curr_y;
        let mut ndigits = 0i32;
        let mut found_x = None;
        let mut found_y = None;

        while curr_y < self.map.height {
            if let Some(Cell::Digit(digit)) = self.map.get(curr_x, curr_y) {
                result = result * 10 + *digit as u32;
                if ndigits == 0 {
                    found_x = Some(curr_x);
                    found_y = Some(curr_y);
                }
                ndigits += 1;
            }
            else if ndigits > 0 {
                break;
            }

            curr_x += 1;
            if curr_x >= self.map.width {
                curr_x = 0;
                curr_y += 1;
                if ndigits > 0 {
                    break;
                }
            }
        }
        if ndigits > 0 {
            self.curr_x = curr_x;
            self.curr_y = curr_y;
            Some((result, MapWindow{ x: found_x.unwrap(), y: found_y.unwrap(), width: ndigits }))
        } else {
            None
        }
    }
}

impl Iterator for MapSymbols<'_> {
    type Item = (char, (i32, i32));

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        let mut curr_x = self.curr_x;
        let mut curr_y = self.curr_y;

        while curr_y < self.map.height {
            if let Some(Cell::Symbol(symbol)) = self.map.get(curr_x, curr_y) {
                result = Some((*symbol, (curr_x, curr_y)));
            }

            curr_x += 1;
            if curr_x >= self.map.width {
                curr_x = 0;
                curr_y += 1;
            }

            if result.is_some() {
                break;
            }
        }
        self.curr_x = curr_x;
        self.curr_y = curr_y;
        result
    }
}

fn parse(input: &str) -> Map {
    let mut cells = Vec::new();
    let mut width = 0i32;
    let mut height = 0i32;

    for line in input.lines() {
        if width == 0 {
            width = line.len() as i32;
        }
        else if width != line.len() as i32 {
            panic!("Invalid input");
        }
        height += 1;
        for c in line.chars() {
            cells.push(match c {
                '.' => Cell::Empty,
                c => {
                    if let Some(x) = c.to_digit(10) {
                        Cell::Digit(x as u8)
                    }
                    else {
                        Cell::Symbol(c)
                    }
                }
            });
        }
    }
    Map { cells, width, height }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input);
    let mut result = 0;
    for (number, window) in map.numbers() {
        let neighbours = map.neighbours(&window);
        if !neighbours.is_empty() {
            result += number;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let numbers = map.numbers().collect::<Vec<_>>();
    let result = map.symbols().filter(|(symbol, _)| *symbol == '*').map(|(_, (x, y))| {
        let mut neighbours = Vec::new();
        for (number, window) in &numbers {
            if window.is_neighbour(x, y) {
                neighbours.push(number);
            }
        }
        if neighbours.len() == 2 {
            neighbours[0] * neighbours[1]
        } else {
            0
        }
    }).sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}

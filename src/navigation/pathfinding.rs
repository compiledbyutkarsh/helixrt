use std::collections::{HashMap, VecDeque};

pub struct NavigationGrid {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}

impl NavigationGrid {
    pub fn new(
        width: usize,
        height: usize,
    ) -> Self {
        Self {
            width,
            height,
            cells: vec![0; width * height],
        }
    }

    pub fn mark_obstacle(
        &mut self,
        x: usize,
        y: usize,
    ) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = y * self.width + x;

        self.cells[index] = 1;
    }

    pub fn occupancy_ratio(&self) -> f64 {
        let occupied =
            self.cells
                .iter()
                .filter(|cell| **cell == 1)
                .count();

        occupied as f64 / self.cells.len() as f64
    }

    pub fn find_path(
        &self,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut frontier =
            VecDeque::new();

        let mut visited =
            HashMap::new();

        frontier.push_back(start);

        visited.insert(start, None);

        let directions = [
            (1isize, 0isize),
            (-1, 0),
            (0, 1),
            (0, -1),
        ];

        while let Some(current) =
            frontier.pop_front()
        {
            if current == goal {
                break;
            }

            for (dx, dy) in directions {
                let nx =
                    current.0 as isize + dx;

                let ny =
                    current.1 as isize + dy;

                if nx < 0 || ny < 0 {
                    continue;
                }

                let next = (
                    nx as usize,
                    ny as usize,
                );

                if next.0 >= self.width
                    || next.1 >= self.height
                {
                    continue;
                }

                if visited.contains_key(&next) {
                    continue;
                }

                frontier.push_back(next);

                visited.insert(
                    next,
                    Some(current),
                );
            }
        }

        let mut path = Vec::new();

        let mut current = goal;

        while let Some(previous) =
            visited.get(&current)
        {
            path.push(current);

            if let Some(node) = previous {
                current = *node;
            } else {
                break;
            }
        }

        path.reverse();

        path
    }
}
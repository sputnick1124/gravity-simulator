use itertools::Itertools;
use std::cell::RefCell;
use std::convert::TryInto;
use std::ops::AddAssign;
use std::rc::Rc;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

type Velocity = Vec3;
type Position = Vec3;

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, other: Velocity) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug)]
struct Body {
    position: Position,
    velocity: Velocity,
}

impl Body {
    fn new(position: Position) -> Self {
        Self {
            position,
            velocity: Velocity::new(),
        }
    }

    fn calc_gravity(&mut self, other: Rc<RefCell<Self>>) {
        let dx = self.position.x - other.borrow().position.x;
        let dy = self.position.y - other.borrow().position.y;
        let dz = self.position.z - other.borrow().position.z;

        self.velocity.x -= dx.signum();
        self.velocity.y -= dy.signum();
        self.velocity.z -= dz.signum();

        other.borrow_mut().velocity.x += dx.signum();
        other.borrow_mut().velocity.y += dy.signum();
        other.borrow_mut().velocity.z += dz.signum();
    }

    fn update_pos(&mut self) {
        self.position += self.velocity;
    }

    fn potential_energy(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs())
            .try_into()
            .unwrap()
    }

    fn kinetic_energy(&self) -> usize {
        (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs())
            .try_into()
            .unwrap()
    }

    fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Debug)]
struct System {
    bodies: Vec<Rc<RefCell<Body>>>,
}

impl System {
    fn new(positions: Vec<Position>) -> Self {
        let bodies = positions
            .into_iter()
            .map(|p| Rc::new(RefCell::new(Body::new(p))))
            .collect();
        Self { bodies }
    }

    fn step(&mut self) {
        for pair in self.bodies.iter().combinations(2) {
            pair[0].borrow_mut().calc_gravity(pair[1].clone());
        }

        self.bodies.iter().for_each(|b| b.borrow_mut().update_pos())
    }

    fn total_energy(&self) -> usize {
        self.bodies.iter().map(|b| b.borrow().total_energy()).sum()
    }

    fn state(&self) -> Vec<isize> {
        let mut vec = Vec::new();
        for body in self.bodies.iter() {
            vec.push(body.borrow().position.x);
            vec.push(body.borrow().position.y);
            vec.push(body.borrow().position.z);
            vec.push(body.borrow().velocity.x);
            vec.push(body.borrow().velocity.y);
            vec.push(body.borrow().velocity.z);
        }
        vec
    }
}

fn main() {
    let positions = vec![
        Position {
            x: -19,
            y: -4,
            z: 2,
        },
        Position {
            x: -9,
            y: 8,
            z: -16,
        },
        Position {
            x: -4,
            y: 5,
            z: -11,
        },
        Position { x: 1, y: 9, z: -13 },
    ];
    let mut system = System::new(positions);

    let mut states = HashSet::new();
    let mut count = 0;
    loop {
        system.step();
        count += 1;
        if count == 1000 {
            println!("Total energy: {}", system.total_energy());
            break; // break here because obviously carrying on is going to fail
        }
        if !states.insert(system.state()) {
            println!("Found a duplicate state after {} iterations", count);
            break;
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let positions = vec![
            Position { x: -1, y: 0, z: 2 },
            Position {
                x: 2,
                y: -10,
                z: -7,
            },
            Position { x: 4, y: -8, z: 8 },
            Position { x: 3, y: 5, z: -1 },
        ];
        let mut system = System::new(positions);

        for _ in 0..10 {
            system.step()
        }
        assert_eq!(system.total_energy(), 179);
    }

    #[test]
    fn example2() {
        let positions = vec![
            Position {
                x: -8,
                y: -10,
                z: 0,
            },
            Position { x: 5, y: 5, z: 10 },
            Position { x: 2, y: -7, z: 3 },
            Position { x: 9, y: -8, z: -3 },
        ];
        let mut system = System::new(positions);

        for _ in 0..100 {
            system.step()
        }
        assert_eq!(system.total_energy(), 1940);
    }
}

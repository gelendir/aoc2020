use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Can be used to represent a coordinate on the map, or a slope delta
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug, PartialEq)]
enum Tile {
    Space,
    Tree
}

/// Coordinate 0,0 represents the top left corner
#[derive(Debug)]
struct Map(Vec<Vec<Tile>>);

impl Point {

    fn read(path: &str) -> Vec<Point> {
        let file = File::open(path).expect("cannot open file");
        let buffer = BufReader::new(file);
        buffer.lines()
            .map(|result| result.expect("cannot read line"))
            .map(|line| Self::parse(&line))
            .collect()
    }

    fn parse(text: &str) -> Point {
        let mut parts = text.split(",");
        let x = parts.next()
            .expect("slope is missing x")
            .parse()
            .expect("cannot parse slope x");

        let y = parts.next()
            .expect("slope is missing y")
            .parse()
            .expect("cannot parse slope y");

        Point{x, y}
    }

    fn new(x: usize, y: usize) -> Point {
        Point {x, y}
    }

    fn off_map(&self, map: &Map) -> bool {
        self.y >= map.height()
    }

    fn advance(&mut self, slope: &Point, map: &Map) {
        self.x += slope.x;
        self.y += slope.y;
        if self.x >= map.width() {
            self.x -= map.width();
        }
    }
}

impl Map {

    fn parse(path: &str) -> Map {
        let file = File::open(path).expect("cannot open file");
        let buffer = BufReader::new(file);
        let tiles = buffer.lines()
            .map(|result| result.expect("cannot read line"))
            .map(|line| {
                line.chars().map(|c| match c {
                    '.' => Tile::Space,
                    '#' => Tile::Tree,
                    _ => panic!("unexpected tile char")
                })
                .collect()
            })
            .collect();
        Map(tiles)
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        //i'm trusting that all rows in the map have the same length
        self.0[0].len()
    }

    fn get(&self, point: &Point) -> &Tile {
        &self.0[point.y][point.x]
    }
}

fn count_trees(map: &Map, slope: &Point) -> usize {
    let mut count = 0;
    let mut point = Point::new(0, 0);

    while !point.off_map(&map) {
        if map.get(&point) == &Tile::Tree {
            count += 1;
        }
        point.advance(&slope, &map);
    }

    count
}

fn main() {
    let path = env::args().nth(1).expect("cannot fild map file path");
    let map = Map::parse(&path);

    let path = env::args().nth(2).expect("cannot find slope file path");
    let slopes = Point::read(&path);

    let trees: Vec<usize> = slopes.iter()
        .map(|slope| count_trees(&map, slope))
        .collect();

    let product: usize = trees.iter().product();

    println!("{:?}", trees);
    println!("{}", product);
}

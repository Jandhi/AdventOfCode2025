use crate::common::{Line2D, Point2d, Rect};
use itertools::Itertools;
use svg::Document;

mod common;

fn is_valid_rect(p1 : Point2d, p2 : Point2d, vertical_slices: &Vec<Line2D>, horizontal_slices: &Vec<Line2D>) -> bool {
    if p1.x > p2.x {
        return false; // Only consider one direction to avoid double counting
    }

    let p3 = Point2d { x: p1.x, y: p2.y };
    let p4 = Point2d { x: p2.x, y: p1.y };

    let rect = Rect {
        p1: p1,
        p2: p2,
    };

    for segment in horizontal_slices {
        if rect.intersects_line(segment) && segment.start.y != p1.y && segment.start.y != p2.y {
            return false;
        }
    }

    for segment in vertical_slices {
        if rect.intersects_line(segment) && segment.start.x != p1.x && segment.start.x != p2.x {
            return false;
        }
    }
    
    return is_in_polygon(p3, &vertical_slices, &horizontal_slices) && is_in_polygon(p4, &vertical_slices, &horizontal_slices);
}

fn is_in_polygon(point : Point2d, y_segments : &Vec<Line2D>, x_segments: &Vec<Line2D>) -> bool {
    let mut intersections = 0;

    // deal with x-aligned segments first
    for line in x_segments {
        if point.y == line.start.y && point.x >= line.start.x.min(line.end.x) && point.x <= line.start.x.max(line.end.x) {
            return true;
        }
    }

    for line in y_segments {
        if point.x == line.start.x && point.y >= line.start.y.min(line.end.y) && point.y <= line.start.y.max(line.end.y) {
            return true;
        }

        if point.y <= line.start.y.min(line.end.y) || point.y > line.start.y.max(line.end.y) {
            continue;
        }

        if point.x < line.start.x || point.x < line.end.x {
            continue;
        }

        if point == line.start || point == line.end {
            return true;
        }

        intersections += 1;
    }

    intersections % 2 == 1
}

fn main() {
    println!("Advent of Code 2025 - Day 9");
    // Your solution here

    let input = include_str!("../input.txt");
    let mut points = input.lines()
        .map(Point2d::from)
        .collect::<Vec<Point2d>>();

    let largest_area = points
        .iter()
        .cartesian_product(points.iter())
        .map(|(p1, p2)| p1.area(p2))
        .max()
        .unwrap();

    println!("p1: {}", largest_area);

    // ensure every 2 points have same x coord
    if points[0].x != points[1].x {
        let p = points.remove(0);
        points.push(p);
    }

    let vertical_slices : Vec<Line2D> = points.iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let p1 = chunk.next().unwrap();
            let p2 = chunk.next().unwrap();
            Line2D { start: *p1, end: *p2 }
        })
        .collect();
    

    // cycle the points 
    let rm = points.remove(0);
    points.push(rm);
    
    let horizontal_slices : Vec<Line2D> = points.iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let p1 = chunk.next().unwrap();
            let p2 = chunk.next().unwrap();
            Line2D { start: *p1, end: *p2 }
        })
        .collect();

    let largest_area_p2 = points
        .iter()
        .cartesian_product(points.iter())
        .filter_map(|(p1, p2)| {
            if !is_valid_rect(*p1, *p2, &vertical_slices, &horizontal_slices) {
                return None;
            }

            Some((*p1, *p2, p1.area(p2)))
        })
        .max_by_key(|(_, _, area)| *area)
        .unwrap();

    let (rect_p1, rect_p2, largest_area_p2) = largest_area_p2;

    let rect = Rect {
        p1: rect_p1,
        p2: rect_p2,
    };
    
    let mut intersecting_horizontal = std::collections::HashSet::new();
    let mut intersecting_vertical = std::collections::HashSet::new();
    
    for (i, seg) in horizontal_slices.iter().enumerate() {
        if rect.intersects_line(seg) {
            println!("Rect intersects horizontal slice");
            println!("Rect: {:?}, Segment: {:?}", rect, seg);
            intersecting_horizontal.insert(i);
        }
    }
    
    for (i, seg) in vertical_slices.iter().enumerate() {
        if rect.intersects_line(seg) {
            intersecting_vertical.insert(i);
        }
    }

    println!("p2: {}", largest_area_p2);

    let scale = 0.01; // Scale down by 100
    let mut document = Document::new()
        .set("viewBox", (-10, -10, 1000, 1000));

        for (i, line2d) in vertical_slices.iter().enumerate() {
            let color = if intersecting_vertical.contains(&i) { "purple" } else { "blue" };
            let line = svg::node::element::Line::new()
                .set("x1", line2d.start.x as f64 * scale)
                .set("y1", line2d.start.y as f64 * scale)
                .set("x2", line2d.end.x as f64 * scale)
                .set("y2", line2d.end.y as f64 * scale)
                .set("stroke", color)
                .set("stroke-width", 1);
            document = document.add(line);
        }

        for (i, line2d) in horizontal_slices.iter().enumerate() {
            let color = if intersecting_horizontal.contains(&i) { "orange" } else { "red" };
            let line = svg::node::element::Line::new()
                .set("x1", line2d.start.x as f64 * scale)
                .set("y1", line2d.start.y as f64 * scale)
                .set("x2", line2d.end.x as f64 * scale)
                .set("y2", line2d.end.y as f64 * scale)
                .set("stroke", color)
                .set("stroke-width", 1);
            document = document.add(line);
        }

        // Draw the largest rectangle
        let rect = svg::node::element::Rectangle::new()
            .set("x", rect_p1.x.min(rect_p2.x) as f64 * scale)
            .set("y", rect_p1.y.min(rect_p2.y) as f64 * scale)
            .set("width", (rect_p1.x - rect_p2.x).abs() as f64 * scale)
            .set("height", (rect_p1.y - rect_p2.y).abs() as f64 * scale)
            .set("fill", "none")
            .set("stroke", "green")
            .set("stroke-width", 2);
        document = document.add(rect);

        svg::save("output.svg", &document).unwrap();
        println!("SVG saved to output.svg");
}

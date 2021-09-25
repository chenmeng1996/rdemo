struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::generics_test::{Point, Point2};

    fn largest_i32(list: &[i32]) -> i32 {
        let mut max = list[0];

        for &item in list {
            if item > max {
                max = item;
            }
        }
    
        max
    }

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut max = list[0];

        for &item in list {
            if item > max {
                max = item;
            }
        }

        max
    }

    // 返回引用
    // TODO 类型不实现Copy
    // fn largest1<T: PartialOrd>(list: &[T]) -> &T {
    //     let mut max = &list[0];

    //     for &item in list.iter() {
    //         if item > *max {
    //             max = &item;
    //         }
    //     }

    //     max
    // }


    #[test]
    fn test_point() {
        let p = Point{ x: 1, y: 2};
        println!("{}", p.x());
    }

    #[test]
    fn test_mixup() {
        let p1 = Point2{x: 1, y: 2.3};
        let p2 = Point2{x: "hello", y: 'c'};
        let p3 = p1.mixup(p2);
        println!("{} {}", p3.x, p3.y);
    }
}
use std::{cmp::{min, max}, cell::RefCell, rc::Rc, ops::Deref};

fn main() {
    let nums: Vec<i32> = [1,4,0,-1,-2,-3,-1,-2].to_vec();
    let res = Solution::find132pattern(nums);
    print!("{}", res);
}

struct Solution {

}

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {

        let first_num = nums[0];

        let root = Rc::new(RefCell::new(Node::root(first_num)));

        let mut high = first_num;
        let mut low = first_num;

        
        for n in &nums[1 ..] {
            // println!("________________________");
            
            // let root_str: String = (&(*root.borrow())).into();

            // println!("{}", root_str);

            // println!("looking for {}", n);


            if root.borrow().include(*n, true) {
                return true;
            }
            

            if *n > high {
                high = *n;
            } else {
                high = *n;
                low = *n;
            }   

            // println!("range {} - {}", low, high);

            root.try_borrow_mut().unwrap().add_range(low, high);
        }

        return false;
       
    }
}

#[derive(Clone)]
struct Node {
    min: i32,
    max: i32,
    left_node: Option<Box<Node>>,
    right_node: Option<Box<Node>>,
    level: i32
}

impl Node {
    fn root(first_value: i32) -> Node {
        Node {
            min: first_value,
            max: first_value,
            left_node: None,
            right_node: None,
            level: 0
        }
    }

    fn is_leaf(&self) -> Result<bool, &str> {
        match (&self.left_node, &self.right_node) {
            (None, None) => Ok(true),
            (Some(_), Some(_)) => Ok(false),
            (_,_) => Err("corrupted node")
        }
    }

    fn maybe_include(&self, value: i32, strict: bool) -> bool {
        match strict {
            true => value > self.min && value < self.max,
            false => value >= self.min && value <= self.max,
        } 
        
    }

    fn include(&self, value: i32, strict: bool) -> bool {
        if self.is_leaf().unwrap() {
            return self.maybe_include(value, strict)
        } else {
            match (&self.right_node, &self.left_node) {
                (None, None) => return false,
                (None, Some(l)) => return l.maybe_include(value, false) && l.include(value, strict),
                (Some(r), None) => return r.maybe_include(value, false) && r.include(value, strict),
                (Some(r), Some(l)) => return (l.maybe_include(value, false) && l.include(value, strict)) || (r.maybe_include(value, false) && r.include(value, strict)),
            }
        }
    }

    fn add_range(&mut self, low: i32, high: i32) -> (i32,i32){
        if let Ok(il) = self.is_leaf() {
            if il {
                if self.include(low, false) {
                    // increase max
                    self.max = max(self.max,high);
                    // println!("increase max to {} for range {} {}", self.max, self.min, self.max);
                    return (self.min, self.max);
                } else if self.include(high, false) {
                    // decrease min
                    self.min = min(self.min,low);
                    return (self.min, self.max);
                } else {
                    // create leafs
                    let new_range: Node = Node {
                        min: low,
                        max: high,
                        left_node: None,
                        right_node: None,
                        level: self.level + 1
                    };

                    let old_range: Node = Node {
                        min: self.min,
                        max: self.max,
                        left_node: None,
                        right_node: None,
                        level: self.level + 1
                    };

                    // print!("split!\n {}\n{}", Into::<String>::into(&new_range), Into::<String>::into(&old_range));

                    // add new range to right or left ?
                    if high < self.min {
                        // println!("LEFT");
                        // new range to left
                        self.left_node = Some(Box::new(new_range));
                        self.right_node = Some(Box::new(old_range));
                    } else {
                        // println!("RIGHT");
                        self.right_node = Some(Box::new(new_range));
                        self.left_node = Some(Box::new(old_range));
                    }

                    self.min = min(self.min, low);
                    self.max = max(self.max, high);

                    if self.right_node.as_ref().unwrap().min <= self.left_node.as_ref().unwrap().max - 1 {
                        // println!("merge {} {}", self.min, self.max);
                        self.right_node = None;
                        self.left_node = None;
                    } 
                    return (self.min, self.max);
                }
            
            } else { 
                self.min = min(self.min, low);
                self.max = max(self.max, high);
                let mut l_node= self.left_node.as_ref().unwrap().clone();
                let mut r_node = self.right_node.as_ref().unwrap().clone();
                let mut l_max: i32 = l_node.max;
                let mut r_min: i32 = r_node.min;
                if low <= self.min && high >= self.max {
                    self.right_node = None;
                    self.left_node = None;
                    return (self.min, self.max);
                } else if low <= l_node.min {
                    l_max = l_node.add_range(low, high).1;
                } else {
                    r_min = r_node.add_range(low, high).0;
                }
                
                if r_min <= l_max - 1 {
                    // println!("merge {} {}", self.min, self.max);
                    self.right_node = None;
                    self.left_node = None;
                } else {
                    self.right_node = Some(r_node);
                    self.left_node = Some(l_node);
                }
            }
        }

        return (self.min, self.max);

    }
}

// impl Into<String> for &Node {
//     fn into(self) -> String {
//         let mut res = format!( " level {} | {}  -  {} | \n", self.level, self.min, self.max);

//         if self.is_leaf().unwrap() {
//             res += "\n_____________\n";
//         } else {
//             let l_str: String = (self.left_node.as_ref().unwrap().as_ref()).into();
//             let r_str: String = (self.right_node.as_ref().unwrap().as_ref()).into();
//             res += &l_str;
//             res += &r_str;
//             res += "\n_____________\n";
//         }

//         return res;

//     }
// }
#![allow(unused_variables)] 
#![allow(unused_mut)] 
#![allow(dead_code)]
#![allow(unused_assignments)]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

//Testing Insert and Find
// fn main(){
        // let u = Tree{data: Some(Box::new(1)), left: None, right: None};
        // let y = Tree{data: Some(Box::new(2)), left: None, right: None};
        // let w = Tree{data: Some(Box::new(1)), left: None, right: Some(Box::new(u))};
        // let mut x = Tree{data: Some(Box::new(3)), left: Some(Box::new(y)), right: Some(Box::new(w))};

        // x.insert(5);
        // let z = x.find(&5);    
        // println!("{}", z);
// }

//Testing PreOrder
fn main(){
    let g = Tree{data: Some(Box::new(7)), left: None, right: None};
    let f = Tree{data: Some(Box::new(14)), left: Some(Box::new(g)), right: None};
    let e = Tree{data: Some(Box::new(13)), left: None, right: Some(Box::new(f))};
    let d = Tree{data: Some(Box::new(12)), left: None, right: None};
    let c = Tree{data: Some(Box::new(4)), left: None, right: None};
    let a = Tree{data: Some(Box::new(8)), left: None, right: None};
    let b = Tree{data: Some(Box::new(11)), left: Some(Box::new(c)), right: Some(Box::new(d))};
    let y = Tree{data: Some(Box::new(9)), left: Some(Box::new(a)), right: Some(Box::new(b))};
    let mut x = Tree{data: Some(Box::new(10)), left: Some(Box::new(y)), right: Some(Box::new(e))};
    let mut output = x.postorder();

    for i in output.iter(){
        println!("{}", i);
    }
}

pub struct Tree<T> {
    data: Option<Box<T>>,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree{data: None, left: None, right: None}
    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
        if self.find(&key){
            return true;
        }

        let temp_key = Some(Box::new(key));
        let child = if temp_key < self.data { &mut self.left } else { &mut self.right };
        match child {
            &mut Some(ref mut tree) => return tree.insert(*temp_key.unwrap()),
            &mut None => *child = Some(Box::new(Tree{data: temp_key, left: None, right: None})),
        };
        
        return false;
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        let mut result = false;
        match self.data{
            None=> result = false,
            Some(ref val)=> 
                
                if **val == *key{
                    return true;
                }else{
                    result = false;
                },
                
        }
        match self.left{
            None => result = false,
            Some(ref left_tree) =>
                if left_tree.find(key){return true},
        }
        match self.right{
            None => result = false,
            Some(ref right_tree) =>
                if right_tree.find(key){return true},
        }
        
    return result
    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        let mut output: Vec<&T> = Vec::new();

        match self.data{
            None => (),
            Some(ref val) => output.push(&val),
        }
        match self.left{
            None => output = output,
            Some(ref left_tree) => 
            if true {
                let mut x = left_tree.preorder();
                for i in x.iter(){
                    output.push(i);
                }
            },
        }
        match self.right{
            None => output = output,
            Some(ref right_tree) => 
            if true{
                let y = right_tree.preorder();
                for j in y.iter(){
                    output.push(j);
                }
            }
        }

        output
    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut output: Vec<&T> = Vec::new();

        match self.left{
            None => output = output,
            Some(ref left_tree) => 
            if true { 
                let mut x = left_tree.inorder();
                for i in x.iter(){
                    output.push(i);
                }
            },
        }
        match self.data{
            None => (),
            Some(ref val) => output.push(&val),
        }
        match self.right{
            None => output = output,
            Some(ref right_tree) => 
            if true{
                let y = right_tree.inorder();
                for j in y.iter(){
                    output.push(j);
                }
            }
        }
        output
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut output: Vec<&T> = Vec::new();

        match self.left{
            None => output = output,
            Some(ref left_tree) => 
            if true { 
                let mut x = left_tree.postorder();
                for i in x.iter(){
                    output.push(i);
                }
            },
        }
        match self.right{
            None => output = output,
            Some(ref right_tree) => 
            if true{
                let y = right_tree.postorder();
                for j in y.iter(){
                    output.push(j);
                }
            }
        }
        match self.data{
            None => (),
            Some(ref val) => output.push(&val),
        }
        output
    }
}

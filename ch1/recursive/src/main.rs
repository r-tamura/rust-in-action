fn i(x: i32) -> i32 {
    x
}

#[derive(Debug)]
enum Tree {
    L(i32),
    B(Box<Tree>, Box<Tree>),
}

fn apply<F: Fn(i32) -> i32, G: Fn(i32, i32) -> i32>(f: &F, g: &G, tree: Tree) -> i32 {
    match tree {
        Tree::L(x) => f(x),
        Tree::B(l, r) => g(apply(f, g, *l), apply(f, g, *r)),
    }
}

fn main() {
    let tree = Tree::B(Box::new(Tree::L(1)), Box::new(Tree::L(2)));
    let result = apply(&i, &|x, y| x + y, tree);

    let x = &1;
    let y = &2;
    println!("{}", *x + *y);

    println!("{:?}", result);
}

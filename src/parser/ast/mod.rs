/*

files:
/src
|- /utils
|  |- mod.rs
|  |- string.rs
|- lib.rs
|- other.rs

-->

Project {
    root: Node {
        file: /src/lib.rs
        nodes: [
            Node {
                file: /src/utils/mod.rs
                nodes: [
                    Node {
                        file: /src/utils/string.rs
                        nodes: []
                    }
                ]
            },
            Node {
                file: /src/other.rs
                nodes: []
            }
        ]
    }
}

*/

pub struct FerrumProjectAst {
    pub root: FerrumProjectAstNode,
}

pub struct FerrumProjectAstNode {
    pub file: FerrumFileAst,
    pub nodes: Vec<FerrumProjectAstNode>,
}

pub struct FerrumFileAst {}


# mini java front-end (scanner & parser)
#### structure
```
 mjava_scanner
    ├── Cargo.lock
    ├── Cargo.toml
    ├── docs
    ├── src
    │   ├── cursor.rs
    │   └── lib.rs
    ├── tests
    │   └── tests.rs
    └── test_sources
```
```
 mjava_parser
   ├── Cargo.lock
   ├── Cargo.toml
   ├── output
   ├── src
   │   ├── cursor.rs
   │   ├── lib.rs
   │   ├── parser.rs
   │   ├── syntax.rs
   │   └── tree_node.rs
   ├── tests
   │   └── tests.rs
   └── test_source

```
####  parser part
parser　部分为递归下降法，参考了pratt parsing 方法
#### scanner part
参考了rust的lexer部分代码
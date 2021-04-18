05.实现存在模块的功能，包括：
- 创建存证；
- 撤销存证；

[官方demo](https://substrate.dev/docs/zh-CN/tutorials/build-a-dapp/)

- [创建存证代码地址](https://github.com/hjijin/rust_practice/blob/main/lesson5/substrate-node-template-3.0.0/pallets/poe/src/lib.rs#L104)
- [撤销存证代码地址](https://github.com/hjijin/rust_practice/blob/main/lesson5/substrate-node-template-3.0.0/pallets/poe/src/lib.rs#L136)

06.为存证模块添加新的功能,转移存证，接受两个参数，一个是内容的哈希值，另外一个是存证的接受账户地址。

- [转移存证地址](https://github.com/hjijin/rust_practice/blob/main/lesson5/substrate-node-template-3.0.0/pallets/poe/src/lib.rs#L203)
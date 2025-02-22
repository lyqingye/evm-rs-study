# EVM 反汇编器和解释器

[中文](README.md) | [English](README_EN.md)

### 简介

这个项目是一个用 Rust 编写的 EVM（以太坊虚拟机）实现，旨在帮助学习和理解以太坊虚拟机的内部工作原理。项目包含了反汇编器和解释器，可以执行 EVM 字节码。

### 特性

- EVM 字节码解释器
- 汇编代码支持
- 内存状态管理
- 基本区块上下文模拟
- 合约创建和执行

### 示例

以下是使用解释器的简单示例：

```rust
fn main() {
    let assembler = asm::Assembler::new();
    let code = assembler
        .asm(
            r#"
        PUSH17 0x67600035600757FE5B60005260086018F3
        PUSH1 0
        MSTORE
        PUSH1 0x11
        PUSH1 0xF
        PUSH1 0
        CREATE
        PUSH1 0
        PUSH1 0
        PUSH1 0
        PUSH1 0
        PUSH1 0
        DUP6
        PUSH2 0xFFFF
        CALL
        STOP
    "#,
        )
        .unwrap();
    let args = vec![];

    let mut state = InMemoryStateDB::new();
    let caller = Address::ZERO;
    state.create_object(caller);
    let contract_address = state.create_contract(caller, code);
    let blk_ctx = BlockContext::new();
    let mut vm = Interpreter::new(Box::new(state), &blk_ctx);
    vm.run(caller, caller, contract_address, args, U256::ZERO)
        .unwrap();
}
```

在这个示例中，我们展示了如何：

1. 创建一个汇编器实例
2. 编译 EVM 汇编代码
3. 初始化状态数据库
4. 创建合约
5. 设置区块上下文
6. 运行 EVM 解释器执行代码

### 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详细信息。


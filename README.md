## Rust的Result类型

`Result` 是 Rust 的核心错误处理机制，它的设计直接体现了 Rust 的 **安全性哲学**。其本质是一个**枚举类型**：

```rust
pub enum Result<T, E> {
    Ok(T),   // 成功时返回的包裹值
    Err(E),  // 失败时返回的错误信息
}
```

Result类型中的数据不能被直接调用，必须首先解开“包装”，即对Result进行处理后才可使用其内部的数据，Result被rust编译器要求必须处理，否则在执行`cargo build`时会报错

**处理 `Result` 的常见方式**

**1. 模式匹配（`match`）**

```rust
let file_result = File::open("data.txt");

match file_result {
    Ok(file) => println!("File opened: {:?}", file),
    Err(error) => eprintln!("Error opening file: {}", error),
}
```

**2. 快捷方法（`unwrap` 与 `expect`）**

- `unwrap()`：成功时返回值，失败时 panic

  ```rust
  let file = File::open("data.txt").unwrap(); // 危险！文件不存在时崩溃
  ```

- `expect()`：类似`unwrap()`，但可附加错误信息

  ```rust
  let file = File::open("data.txt").expect("无法打开 data.txt");
  ```

**3. 错误传播（`?` 运算符）**

将错误向上层传递，适用于函数返回 `Result` 的场景：

```rust
fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("data.txt")?; // 若出错，直接返回 Err
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

**4. 组合方法（`map`、`and_then` 等）**

链式处理结果：

```rust
let parsed_number = "42"
    .parse::<i32>()
    .map(|n| n * 2)          // 若成功，映射为 n*2
    .map_err(|e| e.to_string()); // 转换错误类型
```

**`Result`类型的核心价值**

1. **显式错误处理**
   强制开发者明确处理所有可能的错误路径，避免因忽略错误导致程序在运行时崩溃或进入未定义状态。
2. **类型驱动安全**
   将错误信息编码到类型系统中，确保错误不会像传统异常那样“偷偷”传播，破坏控制流。
3. **零成本抽象**
   `Result` 的运行时开销极低（编译后与手写错误检查代码效率相当），符合 Rust 的零成本抽象原则。

## Rust的依赖机制

对于新引入的包，会**在引入后的第一次build时下载包本体和依赖项并编译**，如果下载并编译成功，则只会进行一次下载，后续执行`cargo build`不会重新下载和编译已有的包

```cmd
E:\Works\rust\dependence_test>cargo build
   Compiling cfg-if v1.0.0
   Compiling byteorder v1.5.0
   Compiling getrandom v0.2.15
   Compiling rand_core v0.6.4
   Compiling zerocopy v0.7.35
   Compiling ppv-lite86 v0.2.20
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling dependence_test v0.1.0 (E:\Works\rust\dependence_test)
    Finished dev [unoptimized + debuginfo] target(s) in 1.24s

E:\Works\rust\dependence_test>cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
```

在编译Cargo项目后，会生成Cargo.lock文件，此后如果没有在TOML文件中手动更改依赖版本，则lock文件不会发生改变；若项目被再次build，则编译器**会首先按照lock文件中的依赖版本下载相关的包**，以确保本项目在任何位置的编译结果相同且可用，除非手动更改指定的依赖

如果想要更新依赖版本，除了手动更新以外，还可以使用`cargo update`命令进行更新，该命令会忽略Cargo.lock文件中的内容，直接更新TOML文件，并重新编译，再将更新后的版本号写入lock文件

```cmd
E:\Works\rust\dependence_test>cargo update
    Updating crates.io index
```

`cargo update`命令不会跨越第二个版本号，例如rand包0.3.14版本进行更新，会更新到rand包0.3版本的最新小版本，例如0.3.23版本；如果想要跨大版本进行更新，则需要手动更改TOML文件

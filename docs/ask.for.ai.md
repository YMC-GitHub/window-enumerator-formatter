- 直接使用window_enumerator::WindowInfo等结构还是自定义比较好？有必要引入window_enumerator吗


```powershell
# sh -c "touch src/{lib,errors,types,utils,models,enumerator}.rs"

# sh -c "touch src/{lib,error,formatter}.rs"
# sh -c "touch src/{lib,error,models}.rs"

sh -c "mkdir -p examples"
sh -c "touch examples/{basic_usage,multiple_formats,custom_templates,advanced_config,with_enumerator}.rs"



# 检查项目结构
cargo check

# 运行测试
cargo test

# 构建项目
cargo build

# 构建文档
cargo doc --open


sh -c "mkdir -p src/core"
sh -c "touch src/core/{mod,error,types,utils}.rs"
# ...

# cargo build > src_build_log.txt 2>&1

# extract the core module while maintaining interface compatibility and preserving the original implementation

git add .
git commit -m "refactor:extract core module"

```

## 模块依赖关系
- 分析当前模块之间的依赖关系图，是否存在循环依赖

## 代码未用警告
- 当前模块是否存在未使用的函数或变量
```powershell
# 检查未使用代码
cargo check
cargo clippy -- -W unused

# 检查特定特性组合
cargo check --no-default-features
cargo check --features "sorting"
cargo check --features "selection" 
cargo check --features "sorting,selection"
```

## 代码注释输出
- 检查代码注释是否符合文档规范

- 在文档注释中添加简单示例代码
- 文档注释使用英文
- 非文档注释使用中文
- 在console中输出信息使用英文

```powershell

# 生成本地文档并在浏览器中打开
cargo doc --open --all-features

# 或者只生成文档不打开
cargo doc --all-features

# 检查文档是否完整
cargo doc --no-deps --all-features

# 运行文档测试
cargo test --doc

# 检查是否有缺失的文档
cargo doc --no-deps --all-features --message-format=short 2>&1 | grep "missing documentation"

# 运行所有文档测试
cargo test --doc --all-features

# 检查链接是否有效
cargo doc --no-deps --all-features --document-private-items
```

```powershell

# 首先验证包
cargo package

# 然后发布
cargo publish


```

## 代码质量 | 格式化
```powershell
# 查看 rustfmt 版本和支持的选项
rustfmt --version
rustfmt --help=config

# 格式化整个项目
cargo fmt

# 格式化特定文件
cargo fmt -- src/enumerator.rs

# 检查格式而不修改
cargo fmt -- --check
```


```powershell
cargo clippy -- -D warnings


# cargo clippy --fix --allow-dirty --allow-staged -- -D warnings


```


## 发布之前检查
```powershell
cargo fmt;cargo fmt -- --check

cargo clippy -- -D warnings
cargo clippy --all-features -- -D warnings

cargo test --all-features

cargo doc --no-deps --all-features
# cargo doc --no-deps --all-features --open

cargo package --allow-dirty
cargo publish --dry-run --registry crates-io --allow-dirty

# git push ghg main
# gh workflow run crate-publish-s.yml -f dry_run=true
# gh run list --workflow=crate-publish-s.yml
```
## 运行示例
```powershell
cargo run --example basic_usage
cargo run --example multiple_formats
cargo run --example custom_templates
cargo run --example advanced_config
cargo run --example with_enumerator --features window-enumerator
```
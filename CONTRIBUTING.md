# 贡献指南

感谢您对易经V14推理引擎项目的关注！我们欢迎各种形式的贡献。

## 如何贡献

### 报告问题

如果您发现bug或有功能建议，请：

1. 搜索现有Issue，确保问题未被报告
2. 创建新Issue，详细描述问题或建议
3. 提供复现步骤（如果是bug）

### 提交代码

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 代码规范

- Rust代码遵循 `rustfmt` 格式
- JavaScript遵循 ES6+ 标准
- 所有代码需要有明确的中文注释

## 开发环境

### 前提条件

- Rust 1.56+
- wasm-pack
- Python 3.8+（用于本地测试）

### 本地开发

```bash
# 克隆仓库
git clone https://github.com/your-username/yijing_v14_web.git
cd yijing_v14_web

# 编译WASM
cd yijing_wasm
wasm-pack build --target web

# 启动本地服务器
cd ..
python -m http.server 8080
```

## 规则说明

引擎采用确定性规则，详见 [规则说明](rules.html)。

### 核心原则

1. **确定性优先**：所有推理规则必须是确定性的，无随机性
2. **可解释性**：每一步推理都应有明确规则依据
3. **一致性**：相同输入必须产生相同输出

## 问题解答

如有问题，请创建Issue或联系维护者。

感谢您的贡献！

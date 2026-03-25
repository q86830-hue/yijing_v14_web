# 易经V14推理引擎

## 项目简介

易经V14推理引擎是一个**可解释、可组合、可验证**的易经六爻推理系统，基于WebAssembly技术将核心算法编译为二进制保护。

## 核心特性

- **确定性推理**：相同输入产生相同输出，结果可复现
- **可解释性**：完整展示推理规则（世应、六亲、吉凶判断）
- **跨平台**：编译为WebAssembly，支持所有主流浏览器
- **高性能**：核心算法以机器码执行

## 项目结构

```
yijing_v14_web/
├── index.html              # 主界面
├── rules.html              # 规则说明文档
├── LICENSE                 # 开源许可证
├── README.md              # 项目说明
├── CONTRIBUTING.md         # 贡献指南
├── .gitignore
└── yijing_wasm/           # 核心引擎（Rust）
    ├── src/
    │   └── lib.rs         # 核心推理逻辑
    ├── Cargo.toml          # Rust项目配置
    └── pkg/               # 编译输出（WebAssembly二进制）
```

## 快速开始

### 使用预编译版本

1. 克隆仓库
2. 使用任意HTTP服务器打开 `index.html`

```bash
python -m http.server 8080
```

访问 <http://localhost:8080/> 即可使用。

### 从源码编译

需要安装 [wasm-pack](https://rustwasm.github.io/wasm-pack/)

```bash
cd yijing_wasm
wasm-pack build --target web
```

## 规则说明

引擎采用以下规则体系：

| 规则 | 说明                          |
| -- | --------------------------- |
| 起卦 | `hash(input) mod 64` 映射到64卦 |
| 动爻 | `卦索引 mod 6` 确定变爻位置          |
| 世应 | 京房八宫体系                      |
| 六亲 | 卦宫五行与爻位五行生克                 |
| 吉凶 | 本卦与变卦五行生克关系                 |

详见 [规则说明](rules.html)

## 核心定位

V14是一个**可解释推理引擎**，而非传统占卜工具。测试集由规则库自动生成，引擎输出与规则预设完全一致（100%内部一致性）。若与特定占卜网站或易学流派结果存在差异，属于规则定义差异，而非引擎错误。

体验地址：[易经数字占卜 V14](https://yj-v14.netlify.app/)

## 开源协议

本项目采用 **Apache License 2.0** 开源协议。

## 致谢

- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust-WebAssembly互操作
- 《周易》- 中华传统易学经典
- 京房八宫体系 - 世应排法参考


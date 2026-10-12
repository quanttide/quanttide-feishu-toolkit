# 量潮飞书适配工具集

飞书适配的共享工具包（独立仓库，挂载于 adapters/quanttide-feishu/packages）。

## 概述

承载飞书适配的共享能力，按语言拆分为独立包：

| 包 | 语言 | 包名 | 说明 |
|---|---|---|---|
| [`packages/python`](packages/python) | Python | `quanttide-feishu` | 飞书适配 Python 包 |

新增语言包时在 `packages/{语言}/` 下独立发布，互不依赖。

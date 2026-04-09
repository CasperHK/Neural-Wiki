# 🧠 Neural-Wiki: Karpathy-style Persistent Brain
一個基於 Andrej Karpathy 持久 Wiki 理念，並進化為多維鏈表結構（Multi-dimensional Linked List）的個人知識管理系統。它不只是文件夾，而是一個模擬大腦神經元連結的動態知識數據庫。

## 🌟 核心理念：知識即神經網絡 (Knowledge as Neural Network)
不同於傳統 RAG 的扁平檢索，本系統將每一條知識定義為一個 Node (神經元)，並透過多維度的 Pointers (指標/連結) 相互交織，形成一個可被 LLM 導航的圖譜架構。
* 原子化儲存：每個 Node 僅承載一個核心概念，避免資訊污染。
* 多維導航：支持「邏輯、時間、類比、因果」等多重鏈表維度。
* 持久化記憶：Markdown 原生存儲，LLM 定期「編譯」與「剪枝」。

## 🏗️ 數據結構定義 (Node Structure)
每個 .md 文件或區塊均遵循以下標準格式，以便 AI 進行路徑遍歷：
```markdown
---
id: <UUID>
type: [Concept | Insight | Event | Entity]
weight: <Float: 0.0-1.0> (活化頻率)
dimensions:
  logical_next: [[Node_B]] # 邏輯推導下游
  historical_prev: [[Node_C]] # 歷史起源上游
  analogy_to: [[Node_D]] # 跨領域類比
  contradicts: [[Node_E]] # 矛盾/反例
tags: #brain/cortex/logic
updated: 2026-04-09
---

# 節點標題
這裡存放原子化的知識內容...

## 關聯證據 / 原始數據
> 引用來源：/raw/research_paper_01.pdf
```

## ⚙️ 工作流程 (The Compiler Loop)
系統運行分為三個主要階段，模仿大腦的編碼、鞏固與檢索：
* Ingest (攝取): 將原始資料（PDF, 筆記, 對話）放入 /raw。
* Compile (編譯):
  1. LLM 提取實體並檢查現有 Wiki 是否存在衝突。
  2. Linking: 自動建立 Multi-dimensional Pointers，將新 Node 插入現有鏈表。
* Traverse (遍歷):
  1. 當用戶提問時，LLM 不進行全庫搜索，而是從起始節點出發，沿著特定維度的鏈條（Linked List）進行「路徑推理」。

## 🚀 快速開始
1. 環境準備
    * 編輯器: 推薦使用 Obsidian 以獲得最佳可視化效果。
    * 核心引擎: 支持 Claude Code 或任何具備 MCP (Model Context Protocol) 能力的 LLM。

2. 安裝自動化腳本 (開發中)
    ```bash
    git clone https://github.com
    cd neural-wiki
    pip install -r requirements.txt
    ```

3. 開始編譯你的大腦
    將你的想法丟入 /inbox，執行：
    ```bash
    python compiler.py --mode merge --depth 2
    ```

## 🛠️ 改進與特性 (Compare to Karpathy's)
| 特性 | Karpathy 原版 Wiki | Neural-Wiki (本項目) |
|---|---|---|
| 儲存單位 | 文件 (File-based) | 原子節點 (Node-based) |
| 連結維度 | 單一雙向連結 (Backlinks) | 多維語意鏈表 (Multi-dim Links) |
| 檢索方式 | 關鍵詞 / 向量檢索 | 路徑遍歷 / 激活擴散 (Spreading) |
| 維護機制 | 手動/簡單整合 | 自動剪枝與加固 (Pruning/Strengthening) |

## 🤝 貢獻
歡迎提交 Pull Request 以優化 LLM 遍歷算法 或 Node 模板規範。

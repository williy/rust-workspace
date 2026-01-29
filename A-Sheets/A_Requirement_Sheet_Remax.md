# NWMA 2026: A-Sheet (Requirement Sheet) - Remax-ish

**プロジェクト名称:** Remax-ish (Rust-powered EMAX ISH)
**タグライン:** A memory-safe, thread-safe, mostly-safe editor — and bugs.

---

## 1. プロジェクト基本情報 (A-00x)
| ID | 項目 | 内容 |
| :--- | :--- | :--- |
| **A-001** | システム名称 | **Remax-ish** |
| **A-002** | 執行境界 (Scope) | WSL2 上の開発環境、および Rust ツールチェーン |
| **A-003** | ライセンス | **GPL-3.0** (Emacs への最大の敬意として) |

## 2. 目的と要件 (A-20x)
### A-201 開発の目的
Lisp による高い拡張性を備えた、Rust 製のモダンなテキストエディタ・プロトタイプの構築。

### A-202 主要機能
* **Lisp Interpreter**: Rust 内蔵型の Lisp エンジンによる設定・拡張。
* **Buffer Management**: Rust のメモリ安全性を活かした高速なテキストバッファ。

---
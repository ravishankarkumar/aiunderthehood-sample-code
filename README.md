# AI Under The Hood: Agentic AI Examples

Welcome to the official code repository for the **Agentic AI** series on [aiunderthehood.com](https://aiunderthehood.com).

This repository is designed to be a hands-on companion to the tutorials. We believe that to truly understand agents, you need to see them implemented across different paradigms. Therefore, **Python and Rust are treated as first-class citizens** in this project.

## 📂 Repository Structure

The project is divided into two independent tracks. Each track contains full implementations of the agentic loops, tool-calling logic, and cognitive architectures discussed in the blog.

* **[Python Track](./python):** Built for rapid prototyping, research, and integration with the vast ecosystem of AI libraries.
* **[Rust Track](./rust):** Built for high-performance, memory-safe, and production-grade agentic systems.

## 🛠️ Global Prerequisites

Before diving into a specific folder, ensure you have your environment configured:

1.  **LLM Setup:** You will need a Google Gemini API Key. Get one for free at [Google AI Studio](https://aistudio.google.com/).
2.  **Environment Variables:** Create a `.env` file in this root directory (or inside the specific language folder) and add your key:
    ```bash
    GEMINI_API_KEY=your_actual_key_here
    ```
3.  **Hardware:** While code is cross-platform, these examples are optimized and tested on **macOS (Apple Silicon)**.

## 🚀 Getting Started

Choose your preferred language and follow the instructions in the respective folder:

| Language | Directory | Getting Started |
| :--- | :--- | :--- |
| **Python** | `./python` | `pip install -r requirements.txt` |
| **Rust** | `./rust` | `cargo build` |

---

## 📖 Related Tutorials

This code follows the learning path on **AI Under The Hood**:
1. [What is an Agent?](https://aiunderthehood.com/agentic-ai/01-foundations/a-what-is-an-agent)
2. [The Cognitive Architecture of Agents](https://aiunderthehood.com/agentic-ai/01-foundations/b-cognitive-architecture)
3. *More coming soon...*

---
*Built with ❤️ for the AI engineering community.*
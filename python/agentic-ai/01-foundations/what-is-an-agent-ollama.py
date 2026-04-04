import json
from pathlib import Path
import ollama

# -----------------------------
# Tool: Count regular files
# -----------------------------
def count_files(directory: str = ".") -> str:
    try:
        path = Path(directory)

        if not path.exists():
            return f"Error: directory '{directory}' does not exist."

        if not path.is_dir():
            return f"Error: '{directory}' is not a directory."

        count = sum(
            1 for item in path.iterdir()
            if item.is_file() and not item.name.startswith(".")
        )

        return str(count)

    except Exception as e:
        return f"Error counting files: {e}"


# -----------------------------
# Agent loop (Ollama version)
# -----------------------------
def run_agent(user_query: str):
    model = "qwen3.5:9b"

    # 🔥 This replaces Gemini's tools=[count_files]
    system_prompt = """
You are a helpful assistant.

You have access to a tool:

Tool: count_files(directory: string)
Description: Count number of regular non-hidden files in a directory.

When the user asks about file counts:
- DO NOT guess
- ALWAYS call the tool

To call the tool, respond ONLY with JSON:

{
  "tool": "count_files",
  "arguments": { "directory": "." }
}

After receiving the tool result, continue normally.
"""

    messages = [
        {"role": "system", "content": system_prompt},
        {"role": "user", "content": user_query},
    ]

    while True:
        response = ollama.chat(
            model=model,
            messages=messages,
        )

        content = response["message"]["content"]
        print("Model:", content, "\n")

        # Try to parse tool call
        try:
            parsed = json.loads(content)

            if parsed.get("tool") == "count_files":
                directory = parsed.get("arguments", {}).get("directory", ".")

                result = count_files(directory)
                print("📦 Tool Result:", result, "\n")

                # Add assistant tool call
                messages.append({"role": "assistant", "content": content})

                # Feed result back (like Gemini internally does)
                messages.append({
                    "role": "user",
                    "content": f"Tool result: {result}. Now answer the original question."
                })

                continue

        except json.JSONDecodeError:
            pass

        # Final response
        return content


# -----------------------------
# Main
# -----------------------------
if __name__ == "__main__":
    query = "Count the number of files in the current folder, and tell me one interesting mathematical fact about that number multiplied by a random prime number."

    answer = run_agent(query)

    print("\nFinal answer:\n")
    print(answer)
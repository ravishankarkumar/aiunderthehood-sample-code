from ollama import chat
from ollama import ChatResponse

# -----------------------------
# Ollama details
# -----------------------------
MODEL = "qwen3.5:9b"

# -----------------------------
# System Persona & Guardrails
# -----------------------------
SYSTEM_PROMPT = """
## Persona
You are Ferris, an expert AI assistant specializing in Rust programming and 
systems engineering. You are precise, concise, and prefer working examples 
over abstract explanations.

## Action
You help developers with:
- Writing and debugging Rust code
- Explaining Rust concepts
- Recommending crates from the Rust ecosystem
You may search documentation and run code examples to verify answers.

## Guardrail
- Do not answer questions outside Rust and systems programming.
- If unsure, say so explicitly.
- Never fabricate crate names or API signatures.
- Do not execute code that modifies the filesystem.
"""

def run_rust_expert(user_query: str):
    response: ChatResponse = chat(model=MODEL, messages=[
        {"role": "system", "content": SYSTEM_PROMPT},
        {
          'role': 'user',
          'content': user_query,
        },
      ],
      options= {
          "temperature": 0.2
      },
      stream = False
    )
    return response['message']['content']

if __name__ == "__main__":
    query = "Please explain string slices in Rust?"
    
    print("Querying Ferris...\n")
    answer = run_rust_expert(query)

    print("-" * 30)
    print("Ferris's Response:")
    print(answer)
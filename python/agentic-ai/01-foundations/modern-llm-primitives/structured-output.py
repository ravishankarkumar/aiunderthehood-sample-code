from ollama import chat
from pydantic import BaseModel

MODEL = "qwen3.5:9b"

# -----------------------------
# Schema
# -----------------------------
class ResearchResult(BaseModel):
    topic: str
    summary: str
    sources: list[str]
    confidence: float
    needs_more_research: bool

# -----------------------------
# Run query
# -----------------------------
response = chat(
    model=MODEL,
    messages=[
        {"role": "system", "content": "You are a research assistant."},
        {"role": "user", "content": "Research the current state of MCP (Model Context Protocol)."},
    ],
    format=ResearchResult.model_json_schema(),
    options={"temperature": 0.2},
)

# -----------------------------
# Parse response
# -----------------------------
result = ResearchResult.model_validate_json(response.message.content)

print(f"Topic: {result.topic}")
print(f"Summary: {result.summary}")
print(f"Confidence: {result.confidence:.0%}")
print(f"Needs more research: {result.needs_more_research}")
print(f"Sources: {result.sources}")
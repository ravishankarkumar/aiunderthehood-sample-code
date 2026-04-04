from __future__ import annotations
from pydantic import BaseModel
from typing import List
from ollama import chat

MODEL = "qwen3.5:9b"

class Step(BaseModel):
    number: int
    description: str

class Plan(BaseModel):
    steps: List[Step]

def generate_plan(goal: str) -> Plan:
    prompt = f"""You are an agent planner.
Break the following goal into a clear, numbered sequence of steps.
Consider tool constraints and potential uncertainties.

Goal: {goal}

Return ONLY valid JSON matching this schema:
{{
  "steps": [
    {{"number": 1, "description": "..."}},
    ...
  ]
}}
"""

    response = chat(
        model=MODEL,
        messages=[{"role": "user", "content": prompt}],
        format=Plan.model_json_schema(),   # forces structured JSON
        options={"temperature": 0.2},
    )

    return Plan.model_validate_json(response.message.content)

if __name__ == "__main__":
    goal = "Find the best laptop GPU choice for local ML and explain why."
    plan = generate_plan(goal)

    print("\n=== GENERATED PLAN ===\n")
    for step in plan.steps:
        print(f"{step.number}. {step.description}")
    print("\n=== END OF PLAN ===\n")
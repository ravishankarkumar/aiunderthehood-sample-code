from __future__ import annotations

import json
from dataclasses import dataclass
from ollama import chat
from pydantic import BaseModel, ValidationError

MODEL = "qwen3.5:9b"
MAX_STEPS = 6


@dataclass
class Step:
    thought: str
    action: str
    observation: str


class AgentAction(BaseModel):
    thought: str
    action: str  # "web_search" | "finish"
    input: str


class Scratchpad:
    def __init__(self, max_steps: int = 8):
        self.max_steps = max_steps
        self.steps: list[Step] = []

    def add(self, thought: str, action: str, observation: str) -> None:
        self.steps.append(Step(thought=thought, action=action, observation=observation))
        # Sliding window to keep working memory bounded.
        self.steps = self.steps[-self.max_steps :]

    def build_prompt(self, goal: str) -> str:
        memory_lines = []
        for s in self.steps:
            memory_lines.append(f"Thought: {s.thought}")
            memory_lines.append(f"Action: {s.action}")
            memory_lines.append(f"Observation: {s.observation}")
            memory_lines.append("")
        memory_text = "\n".join(memory_lines) if memory_lines else "(empty)"
        return f"""
You are an agent planner.
Given GOAL and WORKING MEMORY, return ONLY JSON:
{{
  "thought": string,
  "action": "web_search" | "finish",
  "input": string
}}

GOAL:
{goal}

WORKING MEMORY:
{memory_text}
""".strip()

    def render(self) -> str:
        if not self.steps:
            return "(empty)"
        lines = []
        for i, s in enumerate(self.steps, start=1):
            lines.append(f"{i}. Thought: {s.thought}")
            lines.append(f"   Action: {s.action}")
            lines.append(f"   Observation: {s.observation}")
        return "\n".join(lines)


def web_search(query: str) -> str:
    # Replace with real tool integration if needed.
    return f"[mock-search-result] {query}: RTX 4090 has strong local value; H100 leads datacenter throughput."


def parse_action(raw: str) -> AgentAction:
    """Parse model output robustly even if it includes extra text."""
    try:
        return AgentAction.model_validate_json(raw)
    except ValidationError:
        # Fallback: extract first JSON object from mixed output.
        start = raw.find("{")
        end = raw.rfind("}")
        if start == -1 or end == -1 or end <= start:
            raise ValueError(f"Model returned non-JSON output: {raw[:200]}")
        return AgentAction.model_validate_json(raw[start : end + 1])


def run_agent(goal: str) -> str:
    scratchpad = Scratchpad(max_steps=10)

    for step_no in range(1, MAX_STEPS + 1):
        prompt = scratchpad.build_prompt(goal)
        response = chat(
            model=MODEL,
            messages=[{"role": "user", "content": prompt}],
            format=AgentAction.model_json_schema(),
            options={"temperature": 0.2},
        )

        try:
            action = parse_action(response.message.content)
        except Exception:
            scratchpad.add(
                "Parser failure",
                "finish",
                "Model output was not valid JSON. Retrying with stricter prompt is recommended.",
            )
            print(f"[step {step_no}/{MAX_STEPS}] parse_error")
            print("[scratchpad]")
            print(scratchpad.render())
            return "Stopped: model did not return valid structured output."

        thought = action.thought
        action_name = action.action
        action_input = action.input

        if action_name == "finish":
            scratchpad.add(thought, "finish", action_input)
            print(f"[step {step_no}/{MAX_STEPS}] finished")
            print("[scratchpad]")
            print(scratchpad.render())
            return action_input

        if action_name == "web_search":
            tool_result = web_search(action_input)
            scratchpad.add(thought, f"web_search({action_input})", tool_result)
            print(f"[step {step_no}/{MAX_STEPS}] tool=web_search")
            print("[scratchpad]")
            print(scratchpad.render())
            continue

        scratchpad.add(thought, action_name, "Unknown action. Ask model to finish.")
        print(f"[step {step_no}/{MAX_STEPS}] tool=unknown")
        print("[scratchpad]")
        print(scratchpad.render())

    return "Stopped: reached max iterations without final answer."


if __name__ == "__main__":
    final_answer = run_agent("Find the best laptop GPU choice for local ML and explain why.")
    print("\nFinal answer:")
    print(final_answer)
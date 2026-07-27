from dataclasses import dataclass, field


@dataclass
class Check:
    name: str
    passed: bool
    detail: str


@dataclass
class RunResult:
    run_id: str
    task: str
    timestamp: str
    workspace_path: str
    agent_termination: str
    agent_log: str
    checks: list = field(default_factory=list)
    judge_scores: list = field(default_factory=list)
    overall_pass: bool = False
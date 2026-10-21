"""Extract a deterministic outline from Python source.

This script reads {"files": [{"path": ..., "content": ...}]} from stdin and
writes {"files": [...]} to stdout. It parses source text only; it never imports
or executes project code.
"""

import ast
import io
import json
import sys
import tokenize


def visibility(name: str) -> str:
    return "private" if name.startswith("_") else "public"


def call_fact(node: ast.Call):
    func = node.func
    if isinstance(func, ast.Name):
        return {"callee": func.id, "kind": "function", "qualifier": None}
    if isinstance(func, ast.Attribute):
        receiver = "unknown"
        qualifier = None
        if isinstance(func.value, ast.Name):
            if func.value.id in ("self", "cls"):
                receiver = "self"
            elif func.value.id[:1].isupper():
                receiver = "qualifier"
                qualifier = func.value.id
        return {
            "callee": func.attr,
            "kind": "method",
            "receiver": receiver,
            "qualifier": qualifier,
        }
    try:
        callee = ast.unparse(func)
    except Exception:
        callee = "<call>"
    return {"callee": callee, "kind": "method", "receiver": "unknown", "qualifier": None}


def calls_in(node: ast.AST):
    calls = []
    for child in ast.walk(node):
        if isinstance(child, ast.Call):
            calls.append((child.lineno, child.col_offset, call_fact(child)))
    calls.sort(key=lambda item: (item[0], item[1]))
    return [fact for _, _, fact in calls]


def field_accesses(node: ast.AST):
    names = set()
    for child in ast.walk(node):
        if (
            isinstance(child, ast.Attribute)
            and isinstance(child.value, ast.Name)
            and child.value.id == "self"
        ):
            names.add(child.attr)
    return sorted(names)


def signature(node: ast.AST) -> str:
    prefix = "async def" if isinstance(node, ast.AsyncFunctionDef) else "def"
    text = f"{prefix} {node.name}({ast.unparse(node.args)})"
    if node.returns is not None:
        text += f" -> {ast.unparse(node.returns)}"
    return text


def method(node: ast.AST):
    return {
        "name": node.name,
        "visibility": visibility(node.name),
        "signature": signature(node),
        "start_line": node.lineno,
        "end_line": node.end_lineno or node.lineno,
        "calls": calls_in(node),
        "field_accesses": field_accesses(node),
    }


def class_fields(node: ast.ClassDef):
    fields = {}
    for item in node.body:
        if isinstance(item, ast.AnnAssign) and isinstance(item.target, ast.Name):
            fields.setdefault(
                item.target.id,
                {
                    "name": item.target.id,
                    "visibility": visibility(item.target.id),
                    "type": ast.unparse(item.annotation),
                    "line": item.lineno,
                },
            )
        elif isinstance(item, ast.Assign):
            for target in item.targets:
                if isinstance(target, ast.Name):
                    fields.setdefault(
                        target.id,
                        {
                            "name": target.id,
                            "visibility": visibility(target.id),
                            "type": "",
                            "line": item.lineno,
                        },
                    )
    for item in node.body:
        if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef)):
            for name in field_accesses(item):
                fields.setdefault(
                    name,
                    {
                        "name": name,
                        "visibility": visibility(name),
                        "type": "",
                        "line": item.lineno,
                    },
                )
    return sorted(fields.values(), key=lambda field: (field["line"], field["name"]))


def docstring_lines(tree: ast.AST):
    lines = set()

    def visit(node):
        body = getattr(node, "body", None)
        if (
            isinstance(body, list)
            and body
            and isinstance(body[0], ast.Expr)
            and isinstance(getattr(body[0], "value", None), ast.Constant)
            and isinstance(body[0].value.value, str)
        ):
            start = body[0].lineno
            end = body[0].end_lineno or start
            lines.update(range(start, end + 1))
        for child in ast.iter_child_nodes(node):
            visit(child)

    visit(tree)
    return sorted(lines)


def comment_lines(source: str):
    lines = set()
    try:
        for token in tokenize.generate_tokens(io.StringIO(source).readline):
            if token.type == tokenize.COMMENT:
                lines.add(token.start[0])
    except Exception:
        pass
    return sorted(lines)


def parse_file(entry):
    path = entry["path"]
    source = entry["content"]
    try:
        tree = ast.parse(source, filename=path)
    except SyntaxError as error:
        return {
            "path": path,
            "status": "failed",
            "line": error.lineno or 1,
            "message": str(error),
        }

    units = []
    module_methods = []
    file_functions = []
    for node in tree.body:
        if isinstance(node, ast.ClassDef):
            methods = [
                method(child)
                for child in node.body
                if isinstance(child, (ast.FunctionDef, ast.AsyncFunctionDef))
            ]
            units.append(
                {
                    "id": f"type:{path}::{node.name}",
                    "kind": "type",
                    "type_kind": "class",
                    "name": node.name,
                    "start_line": node.lineno,
                    "end_line": node.end_lineno or node.lineno,
                    "visibility": visibility(node.name),
                    "methods": methods,
                    "fields": class_fields(node),
                    "file_functions": file_functions,
                }
            )
        elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            module_methods.append(method(node))
            file_functions.append(node.name)

    units.append(
        {
            "id": f"module:{path}",
            "kind": "module",
            "name": path,
            "start_line": 1,
            "end_line": max(1, len(source.splitlines())),
            "visibility": "private",
            "methods": module_methods,
            "fields": [],
            "file_functions": file_functions,
        }
    )
    return {
        "path": path,
        "status": "analyzed",
        "comment_lines": comment_lines(source),
        "docstring_lines": docstring_lines(tree),
        "units": units,
    }


def main():
    if sys.version_info < (3, 10):
        json.dump(
            {"error": "python 3.10 or newer is required for the outline adapter"},
            sys.stdout,
        )
        return
    payload = json.load(sys.stdin)
    json.dump({"files": [parse_file(entry) for entry in payload["files"]]}, sys.stdout)


if __name__ == "__main__":
    main()

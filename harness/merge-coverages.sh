#!/bin/sh
set -eu

non_e2e_path=${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}
e2e_path=${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}
output_path=${GIVN_COVERAGE_OUTPUT_PATH:-coverage/cobertura-coverage.xml}

python3 - "$non_e2e_path" "$e2e_path" "$output_path" <<'PY'
import pathlib
import sys
import xml.etree.ElementTree as ET

non_e2e_path, e2e_path, output_path = map(pathlib.Path, sys.argv[1:])
non_e2e = ET.parse(non_e2e_path).getroot()
e2e = ET.parse(e2e_path).getroot()

def total(name):
    return int(non_e2e.attrib.get(name, 0)) + int(e2e.attrib.get(name, 0))

lines_covered = total("lines-covered")
lines_valid = total("lines-valid")
branches_covered = total("branches-covered")
branches_valid = total("branches-valid")
non_e2e.attrib.update({
    "lines-covered": str(lines_covered),
    "lines-valid": str(lines_valid),
    "branches-covered": str(branches_covered),
    "branches-valid": str(branches_valid),
    "line-rate": str(lines_covered / lines_valid if lines_valid else 0.0),
    "branch-rate": str(branches_covered / branches_valid if branches_valid else 0.0),
})

output_path.parent.mkdir(parents=True, exist_ok=True)
ET.ElementTree(non_e2e).write(output_path, encoding="utf-8", xml_declaration=True)
PY

#!/bin/sh
set -eu

non_e2e_path=${GIVN_COVERAGE_NON_E2E_PATH:-coverage/non-e2e-cobertura.xml}
e2e_path=${GIVN_COVERAGE_E2E_PATH:-coverage/e2e-cobertura.xml}
output_path=${GIVN_COVERAGE_OUTPUT_PATH:-coverage/cobertura-coverage.xml}

python3 - "$non_e2e_path" "$e2e_path" "$output_path" <<'PY'
import copy
import pathlib
import sys
import xml.etree.ElementTree as ET

non_e2e_path, e2e_path, output_path = map(pathlib.Path, sys.argv[1:])
non_e2e = ET.parse(non_e2e_path).getroot()
e2e = ET.parse(e2e_path).getroot()


def total(name):
    return int(non_e2e.attrib.get(name, 0)) + int(e2e.attrib.get(name, 0))


def lines_node(parent):
    node = parent.find("./lines")
    if node is None:
        node = ET.SubElement(parent, "lines")
    return node


def merge_line_nodes(target_parent, extra_parent):
    extra_lines = extra_parent.find("./lines")
    if extra_lines is None:
        return

    target_lines = lines_node(target_parent)
    target_by_number = {
        int(line.attrib["number"]): line
        for line in target_lines.findall("./line")
    }
    for extra_line in extra_lines.findall("./line"):
        number = int(extra_line.attrib["number"])
        target_line = target_by_number.get(number)
        if target_line is None:
            target_line = copy.deepcopy(extra_line)
            target_lines.append(target_line)
            target_by_number[number] = target_line
        else:
            target_line.set(
                "hits",
                str(
                    int(target_line.attrib.get("hits", 0))
                    + int(extra_line.attrib.get("hits", 0))
                ),
            )

    target_lines[:] = sorted(
        target_lines.findall("./line"),
        key=lambda line: int(line.attrib["number"]),
    )


def refresh_rate(parent):
    lines = lines_node(parent).findall("./line")
    covered = sum(int(line.attrib.get("hits", 0)) > 0 for line in lines)
    valid = len(lines)
    parent.set("lines-covered", str(covered))
    parent.set("lines-valid", str(valid))
    parent.set("line-rate", str(covered / valid if valid else 0.0))


def merge_class(target_class, extra_class):
    merge_line_nodes(target_class, extra_class)
    target_methods = target_class.find("./methods")
    extra_methods = extra_class.find("./methods")
    if target_methods is not None and extra_methods is not None:
        target_method_list = target_methods.findall("./method")
        extra_method_list = extra_methods.findall("./method")
        for index, extra_method in enumerate(extra_method_list):
            if index < len(target_method_list):
                merge_line_nodes(target_method_list[index], extra_method)
                refresh_rate(target_method_list[index])
            else:
                target_methods.append(copy.deepcopy(extra_method))
    refresh_rate(target_class)


def merge_packages():
    target_packages = non_e2e.find("./packages")
    extra_packages = e2e.find("./packages")
    if target_packages is None or extra_packages is None:
        return

    packages_by_name = {
        package.attrib.get("name"): package
        for package in target_packages.findall("./package")
    }
    for extra_package in extra_packages.findall("./package"):
        name = extra_package.attrib.get("name")
        target_package = packages_by_name.get(name)
        if target_package is None:
            target_packages.append(copy.deepcopy(extra_package))
            packages_by_name[name] = target_packages.findall("./package")[-1]
            continue

        target_classes = target_package.find("./classes")
        extra_classes = extra_package.find("./classes")
        if target_classes is None or extra_classes is None:
            continue
        classes_by_filename = {
            cls.attrib.get("filename"): cls
            for cls in target_classes.findall("./class")
        }
        for extra_class in extra_classes.findall("./class"):
            filename = extra_class.attrib.get("filename")
            target_class = classes_by_filename.get(filename)
            if target_class is None:
                target_classes.append(copy.deepcopy(extra_class))
                classes_by_filename[filename] = target_classes.findall("./class")[-1]
            else:
                merge_class(target_class, extra_class)
        refresh_rate(target_package)


merge_packages()


def class_level_totals(tree):
    # Recount from package/class/lines/line only — never all descendants:
    # class elements also carry <methods> blocks with their own <lines>, and
    # a descendant walk would triple-count them. Keyed by distinct
    # (filename, line number) with hits summed across duplicates; a line
    # counts once and is covered when its merged hits are non-zero.
    hits_by_key = {}
    for package in tree.findall("./packages/package"):
        for cls in package.findall("./classes/class"):
            filename = cls.attrib.get("filename")
            for line in cls.findall("./lines/line"):
                key = (filename, int(line.attrib["number"]))
                hits_by_key[key] = (
                    hits_by_key.get(key, 0) + int(line.attrib.get("hits", 0))
                )
    covered = sum(1 for hits in hits_by_key.values() if hits > 0)
    return covered, len(hits_by_key)


lines_covered, lines_valid = class_level_totals(non_e2e)
if lines_valid == 0:
    # Aggregate-only inputs (no <packages> tree): the union is not
    # computable, so keep the summed totals — a recount would write 0/0
    # and fail the gate's validation.
    lines_covered = total("lines-covered")
    lines_valid = total("lines-valid")
branches_covered = total("branches-covered")
branches_valid = total("branches-valid")

non_e2e.attrib["lines-covered"] = str(lines_covered)
non_e2e.attrib["lines-valid"] = str(lines_valid)
non_e2e.attrib["branches-covered"] = str(branches_covered)
non_e2e.attrib["branches-valid"] = str(branches_valid)
non_e2e.attrib["line-rate"] = str(lines_covered / lines_valid if lines_valid else 0.0)
non_e2e.attrib["branch-rate"] = str(branches_covered / branches_valid if branches_valid else 0.0)

output_path.parent.mkdir(parents=True, exist_ok=True)
ET.ElementTree(non_e2e).write(output_path, encoding="utf-8", xml_declaration=True)
PY

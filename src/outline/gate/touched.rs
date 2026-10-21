//! Changed paths to touched units, including cross-file types and trait
//! implementors.

use crate::outline::classify::Assessment;
use crate::outline::model::{Analysis, RepoPath, TypeKind, TypeRelation, UnitId, UnitKind};
use std::collections::BTreeSet;

pub struct TouchedSet {
    pub units: BTreeSet<UnitId>,
    #[allow(dead_code)]
    pub changed_paths: BTreeSet<RepoPath>,
}

pub fn select(head: &Analysis, base: &Assessment, changed: &BTreeSet<String>) -> TouchedSet {
    let mut units = BTreeSet::new();

    for path in changed {
        for unit in &head.units {
            if unit.declaration_file.as_str() == path
                || unit.files.iter().any(|file| file.as_str() == path)
            {
                units.insert(unit.id.clone());
            }
        }
        for unit in &base.units {
            if unit.files.iter().any(|file| file.as_str() == path) {
                units.insert(unit.unit_id.clone());
            }
        }
        let module = UnitId(format!("module:{path}"));
        if known(head, base, &module) {
            units.insert(module);
        }
    }

    // A touched trait expands to its implementors, one level.
    let mut implementors = BTreeSet::new();
    for id in &units {
        let Some(unit) = head.units.iter().find(|unit| &unit.id == id) else {
            continue;
        };
        if !matches!(unit.kind, UnitKind::Type(TypeKind::Trait)) {
            continue;
        }
        for relation in &unit.relations {
            if let TypeRelation::ImplementedBy(implementor) = relation {
                implementors.insert(implementor.clone());
            }
        }
        for candidate in &head.units {
            if candidate
                .relations
                .contains(&TypeRelation::ImplementsUnit(id.clone()))
            {
                implementors.insert(candidate.id.clone());
            }
        }
    }
    units.extend(implementors);

    // Every touched type also touches its declaration-file module.
    let mut modules = BTreeSet::new();
    for id in &units {
        let Some(unit) = head.units.iter().find(|unit| &unit.id == id) else {
            continue;
        };
        if !matches!(unit.kind, UnitKind::Type(_)) {
            continue;
        }
        let module = UnitId(format!("module:{}", unit.declaration_file));
        if known(head, base, &module) {
            modules.insert(module);
        }
    }
    units.extend(modules);

    TouchedSet {
        units,
        changed_paths: changed.iter().map(RepoPath::new).collect(),
    }
}

fn known(head: &Analysis, base: &Assessment, id: &UnitId) -> bool {
    head.units.iter().any(|unit| &unit.id == id)
        || base.units.iter().any(|unit| &unit.unit_id == id)
}

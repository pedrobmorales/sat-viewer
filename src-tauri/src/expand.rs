use std::collections::HashMap;

use varisat::{CnfFormula, ExtendFormula, Lit};

// "1,2,3" => [ "1,2,3", "1,2,-3", "1,-2,3", "1,-2,-3",
//              "-1,2,3", "-1,2,-3", "-1,-2,3", "-1,-2,-3" ]
//

#[derive(Eq, Hash, PartialEq, Debug)]
struct OctetMapKey {
    var1: isize,
    var2: isize,
    var3: isize,
}

type OctetMap = HashMap<OctetMapKey, u8>;

fn add_to_octet_map(octet_map: &mut OctetMap, clause: &[Lit]) {
    let key = OctetMapKey {
        var1: clause[0].to_dimacs(),
        var2: clause[1].to_dimacs(),
        var3: clause[2].to_dimacs(),
    };
    dbg!(&key);
    use std::collections::hash_map::Entry::{Occupied, Vacant};
    let entry = octet_map.entry(key);
    match entry {
        Occupied(mut x) => {
            *(x.get_mut()) = 10;
        }
        Vacant(x) => {
            x.insert(11);
        }
    }
}

fn get_octet_map(formula: CnfFormula) -> OctetMap {
    let mut octet_map = OctetMap::new();

    for clause in formula.iter() {
        add_to_octet_map(&mut octet_map, clause)
    }
    octet_map
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_octet_map() {
        let mut formula = CnfFormula::new();
        let (aP, bP, cP) = (
            Lit::from_dimacs(1),
            Lit::from_dimacs(2),
            Lit::from_dimacs(3),
        );
        let (aN, bN, cN) = (
            Lit::from_dimacs(-1),
            Lit::from_dimacs(-2),
            Lit::from_dimacs(-3),
        );
        dbg!(&aP, &aN);
        // formula.add_clause(&[aP, bP, cP]);
        formula.add_clause(&[aP, bP, cN]);
        formula.add_clause(&[aP, bN, cP]);
        // formula.add_clause(&[aP, bN, cN]);

        let octet_map = get_octet_map(formula);
        assert_eq!(1, octet_map.len());
    }
}

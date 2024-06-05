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

impl OctetMapKey {
    pub fn index(&self) -> u8 {
        let mut result = 0;
        if self.var1 < 0 {
            result = result | 4;
        }
        if self.var2 < 0 {
            result = result | 2;
        }
        if self.var3 < 0 {
            result = result | 1;
        }
        result
    }
}

type OctetMap = HashMap<OctetMapKey, u8>;

fn add_to_octet_map(octet_map: &mut OctetMap, clause: &[Lit]) {
    let key = OctetMapKey {
        var1: clause[0].to_dimacs().abs(),
        var2: clause[1].to_dimacs().abs(),
        var3: clause[2].to_dimacs().abs(),
    };
    let value = OctetMapKey {
        var1: clause[0].to_dimacs(),
        var2: clause[1].to_dimacs(),
        var3: clause[2].to_dimacs(),
    };
    let index = value.index();
    use std::collections::hash_map::Entry::{Occupied, Vacant};
    let entry = octet_map.entry(key);
    match entry {
        Occupied(mut x) => {
            let val = x.get();
            println!(
                "Occupied: val={}, adding index={} val|index={}",
                val,
                index,
                val | index
            );
            let bit = 1 << index;
            *(x.get_mut()) = val | bit;
        }
        Vacant(x) => {
            let val = index;
            x.insert(1 | index);
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

    fn get_vars() -> (Lit, Lit, Lit, Lit, Lit, Lit) {
        let (var1_positive, var2_positive, var3_positive) = (
            Lit::from_dimacs(1),
            Lit::from_dimacs(2),
            Lit::from_dimacs(3),
        );
        let (var1_negative, var2_negative, var3_negative) = (
            Lit::from_dimacs(-1),
            Lit::from_dimacs(-2),
            Lit::from_dimacs(-3),
        );
        return (
            var1_positive,
            var2_positive,
            var3_positive,
            var1_negative,
            var2_negative,
            var3_negative,
        );
    }
    #[test]
    fn test_octet_map() {
        let (
            var1_positive,
            var2_positive,
            var3_positive,
            var1_negative,
            var2_negative,
            var3_negative,
        ) = get_vars();

        for i in 1..255 {
            println!("Loop iteration: {}", i);
            let mut formula: CnfFormula = CnfFormula::new();

            if i & 1 == 1 {
                formula.add_clause(&[var1_positive, var2_positive, var3_positive]);
            }
            if i & 2 == 1 {
                formula.add_clause(&[var1_positive, var2_positive, var3_negative]);
            }
            if i & 4 == 1 {
                formula.add_clause(&[var1_positive, var2_negative, var3_positive]);
            }
            if i & 8 == 1 {
                formula.add_clause(&[var1_positive, var2_negative, var3_negative]);
            }
            if i & 16 == 1 {
                formula.add_clause(&[var1_negative, var2_positive, var3_positive]);
            }
            if i & 32 == 1 {
                formula.add_clause(&[var1_negative, var2_positive, var3_negative]);
            }
            if i & 64 == 1 {
                formula.add_clause(&[var1_negative, var2_negative, var3_positive]);
            }
            if i & 128 == 1 {
                formula.add_clause(&[var1_negative, var2_negative, var3_negative]);
            }

            let octet_map = get_octet_map(formula);
            assert_eq!(1, octet_map.len());
            let value = octet_map.iter().nth(0).unwrap();
            let num = value.1;
            assert_eq!(i, *num);
        }
    }

    #[test]
    fn test_octet_index() {
        let key = OctetMapKey {
            var1: 1,
            var2: 2,
            var3: 3,
        };
        assert_eq!(0, key.index());

        let key = OctetMapKey {
            var1: 1,
            var2: 2,
            var3: -3,
        };
        assert_eq!(1, key.index());

        let key = OctetMapKey {
            var1: 1,
            var2: -2,
            var3: 3,
        };
        assert_eq!(2, key.index());

        let key = OctetMapKey {
            var1: 1,
            var2: -2,
            var3: -3,
        };
        assert_eq!(3, key.index());

        let key = OctetMapKey {
            var1: -1,
            var2: 2,
            var3: 3,
        };
        assert_eq!(4, key.index());

        let key = OctetMapKey {
            var1: -1,
            var2: 2,
            var3: -3,
        };
        assert_eq!(5, key.index());

        let key = OctetMapKey {
            var1: -1,
            var2: -2,
            var3: 3,
        };
        assert_eq!(6, key.index());

        let key = OctetMapKey {
            var1: -1,
            var2: -2,
            var3: -3,
        };
        assert_eq!(7, key.index());
    }
}

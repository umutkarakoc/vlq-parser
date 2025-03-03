pub type RawMappings = Vec<Vec<Vec<i32>>>;
pub type Mappings = Vec<Vec<Option<(i32, i32, i32, i32)>>>;
use std::collections::HashMap;

pub const COMMA_CHAR: char = ',';
pub const SPACE_CHAR: char = ' ';
pub const SEMICOLON_CHAR: char = ';';
pub const VLQ_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

fn create_lookup_tables() -> (HashMap<char, u8>, Vec<char>) {
    let chars = VLQ_TABLE;
    let mut char_to_integer = HashMap::new();
    let mut integer_to_char = vec![SPACE_CHAR; 65];

    for (i, c) in chars.chars().enumerate() {
        char_to_integer.insert(c, i as u8);
        integer_to_char[i] = c;
    }

    (char_to_integer, integer_to_char)
}

fn decode(string: &str) -> Vec<i32> {
    let (char_to_integer, _) = create_lookup_tables();
    let mut result = Vec::new();
    let mut shift = 0;
    let mut value = 0;

    for c in string.chars() {
        let integer = match char_to_integer.get(&c) {
            Some(&val) => val as i32,
            None => continue,
        };

        let has_continuation_bit = integer & 32;
        let integer = integer & 31;
        value += integer << shift;

        if has_continuation_bit != 0 {
            shift += 5;
        } else {
            let should_negate = value & 1;
            value >>= 1;

            if should_negate != 0 {
                result.push(if value == 0 { -0x80000000 } else { -value });
            } else {
                result.push(value);
            }
            value = 0;
            shift = 0;
        }
    }
    result
}

pub fn parse(mappings: &str) -> Mappings {
    let vlqs = mappings
        .split(SEMICOLON_CHAR)
        .map(|line| line.split(COMMA_CHAR).map(decode).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    process(vlqs)
}

fn process(decoded: RawMappings) -> Mappings {
    let mut source_file_index = 0;
    let mut source_code_line = 0;
    let mut source_code_column = 0;

    decoded
        .into_iter()
        .map(|line| {
            let mut generated_code_column = 0;

            line.into_iter()
                .map(|segment| {
                    if segment.len() == 0 {
                        return None;
                    }
                    generated_code_column += segment[0];

                    source_file_index += segment[1];
                    source_code_line += segment[2];
                    source_code_column += segment[3];

                    Some((
                        generated_code_column,
                        source_file_index,
                        source_code_line,
                        source_code_column,
                    ))
                })
                .collect()
        })
        .collect()
}

pub fn to_source(mappings: &Mappings, line: usize, column: usize) -> Option<(usize, usize)> {
    let Some(line_mappings) = mappings.get(line - 1) else {
        return None;
    };

    for &segment in line_mappings {
        let Some((gen_col, _, src_line, src_col)) = segment else {
            continue;
        };
        if gen_col as usize >= column {
            return Some((((src_line + 1) as usize), src_col as usize));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::{parse, to_source};

    #[test]
    fn test_dummy() {
        let input = ";;;;IAQmB,OAAO,GAAE,MAAM,CAAA;;OARzB,KAAK,MAAA,aAAA,CAAA;OACP,CAAQ,MAAA,8BAAA,CAAA;AAEf,MAAM,MAAM,GAAG,MAAM,CAAC;OAIV,SAAA,MAAA;IAFZ,YAAA,MAAA,EAAA,MAAA,EAAA,cAAA,EAAA,MAAA,GAAA,CAAA,CAAA,EAAA,YAAA,GAAA,SAAA,EAAA,SAAA;;;;;uBAGqC,aAAa,CAAA;;;IAL5B,CAAA;;;;;;;;;;;;;;IAKpB,OAAO,CAAC,QAAQ,CAAU,OAAA,EAAA,MAAM,CAAiB;IAEjD,aAAA;;YACE,GAAG,CAAA,MAAA,EAAA,CAAA;YAAH,GAAG,CAuBF,MAAM,CAAC,MAAM,CAAA,CAAA;;;YAtBZ,MAAM,CAAA,MAAA,EAAA,CAAA;YAAN,MAAM,CAoBL,KAAK,CAAC,MAAM,CAAA,CAAA;;;YAnBX,IAAI,QAAC,UAAU,CAAA,CAAA;YAAf,IAAI,CACD,QAAQ,CAAC,EAAE,CAAA,CAAA;YADd,IAAI,CAED,UAAU,CAAC,UAAU,CAAC,IAAI,CAAA,CAAA;YAF7B,IAAI,CAGD,OAAO,CAAC,GAAG,EAAE;gBACZ,GAAO,CAAA;YACT,CAAC,CAAA,CAAA;;QALH,IAAI,CAAA,GAAA,EAAA,CAAA;;YAMJ,IAAI,QAAC,WAAW,CAAA,CAAA;YAAhB,IAAI,CACD,QAAQ,CAAC,EAAE,CAAA,CAAA;YADd,IAAI,CAED,UAAU,CAAC,UAAU,CAAC,IAAI,CAAA,CAAA;YAF7B,IAAI,CAGD,OAAO,CAAC,GAAG,EAAE;gBACZ,KAAK,CAAC,IAAI,CAAC,MAAM,EAAE,SAAS,EAAE,8BAA8B,EAAE,EAAS,GAAG,CAAC,CAAC,EAAE,CAAC,CAAC,CAAC,CAAC;YACpF,CAAC,CAAA,CAAA;;QALH,IAAI,CAAA,GAAA,EAAA,CAAA;;YAMJ,IAAI,QAAC,WAAW,CAAA,CAAA;YAAhB,IAAI,CACD,QAAQ,CAAC,EAAE,CAAA,CAAA;YADd,IAAI,CAED,UAAU,CAAC,UAAU,CAAC,IAAI,CAAA,CAAA;YAF7B,IAAI,CAGD,OAAO,CAAC,GAAG,EAAE;gBACZ;oBAAO,CAAC;YACV,CAAC,CAAA,CAAA;;QALH,IAAI,CAAA,GAAA,EAAA,CAAA;QAbN,MAAM,CAAA,GAAA,EAAA,CAAA;QADR,GAAG,CAAA,GAAA,EAAA,CAAA;IAwBJ,CAAA;;;;;;;;AAGH;IACE,IAAO,CAAA;AACT,CAAC;AAED;IACE,IAAO,CAAA;AACT,CAAC;AACD;IACE,MAAM,KAAK,CAAC,MAAM,CAAC,CAAA;AACrB,CAAC;";
        let mappings = parse(input);
        let target = to_source(&mappings, 54, 1);
        assert!(target == Some((20, 8)));

        let target = to_source(&mappings, 52, 1);
        println!("{:?}", target);
        assert!(target == None);

        let target = to_source(&mappings, 40, 1);
        println!("{:?}", target);
        assert!(target == Some((13, 6)));
    }
}

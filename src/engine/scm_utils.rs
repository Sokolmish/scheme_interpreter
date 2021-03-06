use crate::*;

pub fn scm_is_list(val: &ScmValue) -> bool {
    let mut cur = val;
    if matches!(cur, ScmValue::Nil) {
        return true;
    }
    loop {
        if let ScmValue::DotPair(_, cdr) = cur {
            if matches!(**cdr, ScmValue::Nil) {
                return true;
            }
            cur = cdr;
        } else {
            return false;
        }
    }
}

pub fn scm_list_len(val: &ScmValue) -> Option<i64> {
    let mut cur = val;
    if matches!(cur, ScmValue::Nil) {
        return Some(0);
    }
    let mut res = 1i64;
    loop {
        if let ScmValue::DotPair(_, cdr) = cur {
            if matches!(**cdr, ScmValue::Nil) {
                return Some(res);
            }
            res += 1;
            cur = cdr;
        } else {
            return None;
        }
    }
}

pub fn scm_list_to_vec(list: &ScmValue) -> Vec<ScmValue> {
    if matches!(list, ScmValue::Nil) {
        return Vec::new();
    }

    let mut res = Vec::new();
    let mut cur = list;
    loop {
        match cur {
            ScmValue::Nil => {
                break;
            }
            ScmValue::DotPair(car, cdr) => {
                res.push(*car.clone());
                cur = cdr;
            }
            _ => {
                panic!("Not a list")
            }
        }
    }
    res
}

pub fn scm_is_true(val: &ScmValue) -> bool {
    match val {
        ScmValue::Bool(val) => *val,
        _ => true,
    }
}

pub fn scm_is_false(val: &ScmValue) -> bool {
    !scm_is_true(val)
}

pub fn scm_get_float(val: &ScmValue) -> Option<f64> {
    match *val {
        ScmValue::Number(val) => Some(val.to_float()),
        _ => None,
    }
}

pub fn scm_get_integer(val: &ScmValue) -> Option<i64> {
    match *val {
        ScmValue::Number(val) => Some(val.to_int()),
        _ => None,
    }
}

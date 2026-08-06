use crate::definitions::use_cases::exiter::Exit;

pub fn exit() {
    let exit: Exit = noop;

    exit_with(exit);
}

pub(crate) fn exit_with(exit: Exit) {
    exit();
}

fn noop() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exiter_matches_use_case_function_pointer() {
        let exit_fn: Exit = exit;

        let _ = exit_fn;
    }
}

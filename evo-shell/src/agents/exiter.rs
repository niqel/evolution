use crate::definitions::use_cases::exiter::Exit;

pub fn exit() {
    let exit: Exit = noop;

    exit_with(exit);
}

pub(crate) fn exit_with(exit: Exit) {
    exit();
}

fn noop() {}

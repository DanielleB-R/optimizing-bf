use crate::syntax::BFSyntax;

fn coalesce_constants(input: Vec<BFSyntax>) -> Vec<BFSyntax> {
    let mut result = vec![];
    let mut current: Option<BFSyntax> = None;

    for instruction in input {
        match instruction {
            BFSyntax::Root(_) => unimplemented!(),
            BFSyntax::Read => {
                if let Some(inst) = current.take() {
                    result.push(inst);
                }
                result.push(BFSyntax::Read);
            }
            BFSyntax::Write => {
                if let Some(inst) = current.take() {
                    result.push(inst);
                }
                result.push(BFSyntax::Write);
            }
            BFSyntax::Loop(contents) => {
                if let Some(inst) = current.take() {
                    result.push(inst);
                }
                result.push(BFSyntax::Loop(coalesce_constants(contents)));
            }
            BFSyntax::Inc(n) => match current.take() {
                None => {
                    current = Some(BFSyntax::Inc(n));
                }
                Some(BFSyntax::Inc(m)) => {
                    current = Some(BFSyntax::Inc(n + m));
                }
                Some(BFSyntax::Dec(m)) => {
                    current = Some(if n > m {
                        BFSyntax::Inc(n - m)
                    } else {
                        BFSyntax::Dec(m - n)
                    })
                }
                Some(inst) => {
                    result.push(inst);
                    current = Some(BFSyntax::Inc(n));
                }
            },
            BFSyntax::Dec(n) => match current.take() {
                None => {
                    current = Some(BFSyntax::Dec(n));
                }
                Some(BFSyntax::Dec(m)) => {
                    current = Some(BFSyntax::Dec(n + m));
                }
                Some(BFSyntax::Inc(m)) => {
                    current = Some(if n > m {
                        BFSyntax::Dec(n - m)
                    } else {
                        BFSyntax::Inc(m - n)
                    })
                }
                Some(inst) => {
                    result.push(inst);
                    current = Some(BFSyntax::Dec(n));
                }
            },
            BFSyntax::Right(n) => match current.take() {
                None => {
                    current = Some(BFSyntax::Right(n));
                }
                Some(BFSyntax::Right(m)) => {
                    current = Some(BFSyntax::Right(n + m));
                }
                Some(BFSyntax::Left(m)) => {
                    current = Some(if n > m {
                        BFSyntax::Right(n - m)
                    } else {
                        BFSyntax::Left(m - n)
                    })
                }
                Some(inst) => {
                    result.push(inst);
                    current = Some(BFSyntax::Right(n));
                }
            },
            BFSyntax::Left(n) => match current.take() {
                None => {
                    current = Some(BFSyntax::Left(n));
                }
                Some(BFSyntax::Left(m)) => {
                    current = Some(BFSyntax::Left(n + m));
                }
                Some(BFSyntax::Right(m)) => {
                    current = Some(if n > m {
                        BFSyntax::Left(n - m)
                    } else {
                        BFSyntax::Right(m - n)
                    })
                }
                Some(inst) => {
                    result.push(inst);
                    current = Some(BFSyntax::Left(n));
                }
            },
        }
    }
    if let Some(inst) = current.take() {
        result.push(inst);
    }

    result
}

pub fn fold_adjacent_constants(input: BFSyntax) -> BFSyntax {
    if let BFSyntax::Root(instructions) = input {
        return BFSyntax::Root(coalesce_constants(instructions));
    }
    unimplemented!();
}

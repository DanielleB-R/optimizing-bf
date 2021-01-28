use crate::syntax::BFSyntax;

fn coalesce_constants(input: Vec<BFSyntax>) -> Vec<BFSyntax> {
    let mut result = vec![];
    let mut current: Option<BFSyntax> = None;

    for instruction in input {
        match instruction {
            BFSyntax::Root(_) => unimplemented!(),
            BFSyntax::Read => {
                match current.take() {
                    // Read will clobber the results of these
                    Some(BFSyntax::Inc(_))
                    | Some(BFSyntax::Dec(_))
                    | Some(BFSyntax::Set(_))
                    | None => {}
                    Some(inst) => {
                        result.push(inst);
                    }
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
                    });
                }
                Some(BFSyntax::Set(m)) => {
                    current = Some(BFSyntax::Set(n.wrapping_add(m)));
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
                Some(BFSyntax::Set(m)) => {
                    current = Some(BFSyntax::Set(n.wrapping_sub(m)));
                }
                Some(inst) => {
                    result.push(inst);
                    current = Some(BFSyntax::Dec(n));
                }
            },
            BFSyntax::Set(n) => {
                match current.take() {
                    // discard the saved instruction if it alters the current cell since the result is overwritten
                    Some(BFSyntax::Dec(_))
                    | Some(BFSyntax::Inc(_))
                    | Some(BFSyntax::Set(_))
                    | None => {}
                    Some(inst) => {
                        result.push(inst);
                    }
                }
                current = Some(BFSyntax::Set(n));
            }
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

pub fn optimize_zeroing_loops(input: BFSyntax) -> BFSyntax {
    match input {
        BFSyntax::Root(insts) => {
            BFSyntax::Root(insts.into_iter().map(optimize_zeroing_loops).collect())
        }
        BFSyntax::Loop(insts) => {
            if insts.len() == 1 && insts[0] == BFSyntax::Dec(1) {
                BFSyntax::Set(0)
            } else {
                BFSyntax::Loop(insts.into_iter().map(optimize_zeroing_loops).collect())
            }
        }
        inst => inst,
    }
}

pub fn strip_dead_loops(input: BFSyntax) -> BFSyntax {
    match input {
        BFSyntax::Root(insts) => {
            let mut last_is_loop = false;
            BFSyntax::Root(
                insts
                    .into_iter()
                    .skip_while(BFSyntax::is_loop)
                    .filter_map(|inst| {
                        if inst.is_loop() {
                            if last_is_loop {
                                None
                            } else {
                                last_is_loop = true;
                                Some(strip_dead_loops(inst))
                            }
                        } else {
                            last_is_loop = false;
                            Some(inst)
                        }
                    })
                    .collect(),
            )
        }
        BFSyntax::Loop(insts) => {
            let mut last_is_loop = false;
            BFSyntax::Loop(
                insts
                    .into_iter()
                    .filter_map(|inst| {
                        if inst.is_loop() {
                            if last_is_loop {
                                None
                            } else {
                                last_is_loop = true;
                                Some(strip_dead_loops(inst))
                            }
                        } else {
                            last_is_loop = false;
                            Some(inst)
                        }
                    })
                    .collect(),
            )
        }
        inst => inst,
    }
}

pub fn perform_all(input: BFSyntax) -> BFSyntax {
    fold_adjacent_constants(optimize_zeroing_loops(strip_dead_loops(input)))
}

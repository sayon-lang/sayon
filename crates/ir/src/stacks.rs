/*!
Which arrays a function keeps on a data stack.

Lowering puts every array local on a data stack, with its scope's mark
(`Function::stack_values`). An array stays there when the function makes it
and keeps it to itself, so that its storage can be freed with its scope:

- it is given its value once, by an array literal, so a loop that gives it a
  new array each time round can't pile them up on the stack;
- it is never moved, whole, anywhere else: not returned, not stored in
  another value, not passed to a parameter that takes it over, not captured
  by a closure, and not sent through a channel.

IMPORTANT: an array that stays on a stack must not outlive its scope, which
these two rules ensure, since a value can only leave its scope by being
moved. Every other array keeps its storage on the heap. Guarded by the test
programs `arrays_on_stacks.sayon` and `arrays_that_leave.sayon`.
*/

use crate::ir::*;
use std::collections::{HashMap, HashSet};

/** The array locals of `function` that stay on their data stacks. */
pub fn placed(function: &Function) -> Vec<StackValue> {
    if function.stack_values.is_empty() {
        return Vec::new();
    }
    let mut uses = Uses::default();
    for block in &function.blocks {
        for inst in &block.insts {
            uses.inst(inst);
        }
        match &block.term {
            Terminator::Op { op, dest, .. } => {
                if let Some(dest) = dest {
                    uses.given(dest, false);
                }
                uses.op(op);
            }
            Terminator::Branch { condition, .. } => uses.operand(condition),
            Terminator::Goto(_)
            | Terminator::Return
            | Terminator::ReturnError
            | Terminator::Unreachable => {}
        }
    }
    function
        .stack_values
        .iter()
        .filter(|value| {
            uses.given.get(&value.value) == Some(&(1, true)) && !uses.moved.contains(&value.value)
        })
        .copied()
        .collect()
}

/** What a function does with its slots, as far as placing them goes. */
#[derive(Default)]
struct Uses {
    /** For each slot given a whole value: how many times, and whether always by an array literal. */
    given: HashMap<SlotId, (usize, bool)>,
    /** The slots whose whole value is moved somewhere. */
    moved: HashSet<SlotId>,
}

/** The slot a place is, when it is a whole slot rather than a part of one. */
fn whole(place: &Place) -> Option<SlotId> {
    match (&place.base, place.projection.is_empty()) {
        (Base::Slot(slot), true) => Some(*slot),
        _ => None,
    }
}

impl Uses {
    fn given(&mut self, dest: &Place, by_literal: bool) {
        if let Some(slot) = whole(dest) {
            let entry = self.given.entry(slot).or_insert((0, true));
            entry.0 += 1;
            entry.1 &= by_literal;
        }
    }

    fn operand(&mut self, operand: &Operand) {
        if let Operand::Move(place) = operand
            && let Some(slot) = whole(place)
        {
            self.moved.insert(slot);
        }
    }

    fn operands<'a>(&mut self, operands: impl IntoIterator<Item = &'a Operand>) {
        for operand in operands {
            self.operand(operand);
        }
    }

    fn inst(&mut self, inst: &Inst) {
        match inst {
            Inst::Assign { dest, value } => {
                self.given(dest, matches!(value, Rvalue::MakeArray { .. }));
                self.rvalue(value);
            }
            Inst::Push { value, .. } | Inst::Append { value, .. } => self.operand(value),
            Inst::DictSet { key, value, .. } => self.operands([key, value]),
            Inst::MutexPut { mutex, value } => self.operands([mutex, value]),
            Inst::Bind { .. }
            | Inst::Drop(_)
            | Inst::StackMark { .. }
            | Inst::StackRestore { .. } => {}
        }
    }

    fn rvalue(&mut self, rvalue: &Rvalue) {
        match rvalue {
            Rvalue::Use(operand)
            | Rvalue::Not(operand)
            | Rvalue::FloatNegate(operand)
            | Rvalue::UnionTypeName(operand)
            | Rvalue::ByteLength(operand)
            | Rvalue::DictSlots(operand)
            | Rvalue::ErrorMessage(operand)
            | Rvalue::ErrorCode(operand)
            | Rvalue::RangeLength(operand)
            | Rvalue::Increment(operand)
            | Rvalue::Downgrade(operand)
            | Rvalue::MakeUnion { value: operand, .. }
            | Rvalue::VariantIs { value: operand, .. }
            | Rvalue::TagIs { value: operand, .. }
            | Rvalue::Length { value: operand, .. }
            | Rvalue::Clone { value: operand, .. }
            | Rvalue::MakeShared { value: operand, .. }
            | Rvalue::MakeMutex { value: operand, .. }
            | Rvalue::Upgrade { weak: operand, .. }
            | Rvalue::RemoveKey { key: operand, .. }
            | Rvalue::NextChar { text: operand, .. } => self.operand(operand),
            Rvalue::FloatArith(_, lhs, rhs)
            | Rvalue::Compare { lhs, rhs, .. }
            | Rvalue::Concat(lhs, rhs)
            | Rvalue::DictNext {
                dict: lhs,
                index: rhs,
            }
            | Rvalue::HasKey {
                dict: lhs,
                key: rhs,
            }
            | Rvalue::RangeAt {
                range: lhs,
                index: rhs,
            } => self.operands([lhs, rhs]),
            Rvalue::Format(parts) => {
                for part in parts {
                    if let FormatPart::Value(operand, _) = part {
                        self.operand(operand);
                    }
                }
            }
            Rvalue::MakeArray { items, .. } => self.operands(items),
            Rvalue::MakeDict { entries, .. } => {
                for (key, value) in entries {
                    self.operands([key, value]);
                }
            }
            Rvalue::MakeRecord { fields, .. } | Rvalue::MakeVariant { fields, .. } => {
                self.operands(fields)
            }
            Rvalue::MakeClosure { captures, .. } => self.operands(captures),
            Rvalue::Pop { .. } => {}
        }
    }

    fn op(&mut self, op: &Op) {
        match op {
            Op::Call { args, .. } | Op::CallValue { args, .. } | Op::Host { args, .. } => {
                self.operands(args)
            }
            Op::IntArith { lhs, rhs, .. } => self.operands([lhs, rhs]),
            Op::IntNegate { value, .. }
            | Op::Convert { value, .. }
            | Op::Spawn { body: value, .. }
            | Op::SpawnParallel { body: value, .. }
            | Op::MutexTake { mutex: value }
            | Op::Sleep { nanoseconds: value }
            | Op::ChannelNew {
                capacity: value, ..
            }
            | Op::Receive { channel: value }
            | Op::Close { channel: value }
            | Op::Select {
                channels: value, ..
            }
            | Op::SplitView { count: value, .. }
            | Op::RemoveAt { index: value, .. }
            | Op::ReadFile(value)
            | Op::Fail(value)
            | Op::Raise(value)
            | Op::ToNumber { text: value, .. } => self.operand(value),
            Op::CheckIndex { array, index } => self.operands([array, index]),
            Op::MakeView { start, end, .. } => self.operands(start.iter().chain(end)),
            Op::Send {
                channel, message, ..
            } => {
                self.operand(channel);
                if let Some(slot) = whole(message) {
                    self.moved.insert(slot);
                }
            }
            Op::Swap { first, second, .. } => self.operands([first, second]),
            Op::Exchange { first, second, .. } => self.operands([first, second]),
            Op::CharAt { text, index } => self.operands([text, index]),
            Op::DictFind { dict, key } => self.operands([dict, key]),
            Op::Print(values) => self.operands(values.iter().map(|(operand, _)| operand)),
            Op::Assert { condition, message } => {
                self.operands(std::iter::once(condition).chain(message))
            }
            Op::MakeRange { start, end, step } => {
                self.operands([start, end].into_iter().chain(step))
            }
            Op::WriteFile { path, content } => self.operands([path, content]),
            Op::CallForeign { args, .. } => self.operands(args.iter().map(|(operand, _)| operand)),
            Op::RawRead {
                address, offset, ..
            } => self.operands([address, offset]),
            Op::RawWrite {
                address,
                offset,
                value,
                ..
            } => self.operands([address, offset, value]),
            Op::Join { .. } | Op::Detach { .. } | Op::Yield => {}
        }
    }
}

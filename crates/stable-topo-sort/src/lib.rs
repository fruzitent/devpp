pub mod error;

use std::collections::HashMap;
use std::hash::Hash;

use crate::error::Error;
use crate::error::Result;

/// @see: https://en.wikipedia.org/wiki/Topological_sorting#Depth-first_search
pub fn stable_topo_sort<T>(nodes: &[T], edges: &[(T, T)]) -> Result<Vec<T>>
where
    T: Clone + Eq + Hash,
{
    let mut result = Vec::with_capacity(nodes.len());
    let mut ctx = Context {
        incoming: &edges.iter().fold(Default::default(), |mut acc, (from, to)| {
            acc.entry(to).or_default().push(from);
            acc
        }),
        marks: &mut Default::default(),
        result: &mut result,
    };
    for node in nodes {
        visit(&mut ctx, node)?;
    }
    Ok(result)
}

struct Context<'a, T> {
    incoming: &'a HashMap<&'a T, Vec<&'a T>>,
    marks: &'a mut HashMap<&'a T, Mark>,
    result: &'a mut Vec<T>,
}

enum Mark {
    Permanent,
    Temporary,
}

fn visit<'a, T>(ctx: &mut Context<'a, T>, node: &'a T) -> Result<()>
where
    T: Clone + Eq + Hash,
{
    match ctx.marks.get(node) {
        Some(Mark::Permanent) => return Ok(()),
        Some(Mark::Temporary) => return Err(Error::CycleDetected),
        None => {}
    }
    ctx.marks.insert(node, Mark::Temporary);
    if let Some(deps) = ctx.incoming.get(node) {
        for dep in deps {
            visit(ctx, dep)?;
        }
    }
    ctx.marks.insert(node, Mark::Permanent);
    ctx.result.push(node.clone());
    Ok(())
}

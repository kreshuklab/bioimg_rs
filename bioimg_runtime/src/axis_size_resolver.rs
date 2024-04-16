use std::collections::{BTreeMap, HashMap, HashSet};

use bioimg_spec::rdf::model::{
    axis_size::{QualifiedAxisId, ResolvedAxisSize}, AnyAxisSize, AxisSizeReference, ParameterizedAxisSize
};

pub trait ResolvedAxisSizeExt{
    fn is_compatible_with_extent(&self, extent: usize) -> bool;
}

impl ResolvedAxisSizeExt for ResolvedAxisSize{
    fn is_compatible_with_extent(&self, extent: usize) -> bool {
        match self {
            Self::Fixed(fixed) => return usize::from(*fixed) == extent,
            Self::Parameterized(ParameterizedAxisSize { min, step }) => {
                let min = usize::from(*min);
                let step = usize::from(*step);
                return (extent - min) % step == 0;
            }
        }
    }
}

pub struct SlotResolver {
    resolved_axes: HashMap<QualifiedAxisId, ResolvedAxisSize>,
    unresolved_axes: BTreeMap<QualifiedAxisId, AxisSizeReference>,
}

pub enum ResolverStatus {
    Done(HashMap<QualifiedAxisId, ResolvedAxisSize>),
    Resolving(SlotResolver),
}

#[derive(thiserror::Error, Debug)]
pub enum AxisSizeResolutionError {
    #[error("Loop detected when trying to resolve reference to {0}")]
    Loop(QualifiedAxisId),
    #[error("Reference to {0} is unresolvable")]
    Unresolvable(QualifiedAxisId),
    #[error("Multiple axes with same ID: {0}")]
    DuplicateId(QualifiedAxisId),
    #[error("Parameterized size not allowed")]
    ParameterizedNotAllowed,
}

impl SlotResolver {
    pub fn new(sizes: Vec<(QualifiedAxisId, AnyAxisSize)>) -> Result<Self, AxisSizeResolutionError> {
        let mut resolved_axes: HashMap<QualifiedAxisId, ResolvedAxisSize> = HashMap::with_capacity(sizes.len());
        let mut unresolved_axes: BTreeMap<QualifiedAxisId, AxisSizeReference> = BTreeMap::new();
        for (qual_id, inp_size) in sizes.into_iter() {
            let duplicate_detected = match inp_size {
                AnyAxisSize::Reference(size_ref) => {
                    unresolved_axes.insert(qual_id.clone(), size_ref.clone()).is_some()
                },
                AnyAxisSize::Parameterized(resolved_size) => {
                    resolved_axes.insert(qual_id.clone(), resolved_size.into()).is_some()
                },
                AnyAxisSize::Fixed(resolved_size) => {
                    resolved_axes.insert(qual_id.clone(), resolved_size.into()).is_some()
                }
            };
            if duplicate_detected {
                return Err(AxisSizeResolutionError::DuplicateId(qual_id));
            }
        }
        Ok(Self { resolved_axes, unresolved_axes })
    }

    fn try_resolve(
        &mut self,
        current: QualifiedAxisId,
        mut visited: HashSet<QualifiedAxisId>,
    ) -> Result<ResolvedAxisSize, AxisSizeResolutionError> {
        if let Some(resolved) = self.resolved_axes.get(&current) {
            return Ok(resolved.clone());
        }
        if !visited.insert(current.clone()) {
            return Err(AxisSizeResolutionError::Loop(current));
        }
        let Some(size_ref) = self.unresolved_axes.get(&current) else {
            return Err(AxisSizeResolutionError::Unresolvable(current));
        };
        let resolved = self.try_resolve(size_ref.qualified_axis_id.clone(), visited)?;
        self.unresolved_axes.remove(&current);
        self.resolved_axes.insert(current.clone(), resolved.clone());
        Ok(resolved)
    }

    fn step(mut self) -> Result<ResolverStatus, AxisSizeResolutionError> {
        let Some((key, _)) = self.unresolved_axes.last_key_value() else{
            return Ok(ResolverStatus::Done(self.resolved_axes));
        };
        self.try_resolve(key.clone(), HashSet::new())?;
        Ok(ResolverStatus::Resolving(self))
    }

    pub fn solve(mut self) -> Result<HashMap<QualifiedAxisId, ResolvedAxisSize>, AxisSizeResolutionError> {
        loop {
            match self.step()? {
                ResolverStatus::Done(map) => break Ok(map),
                ResolverStatus::Resolving(resolver) => {
                    self = resolver;
                }
            }
        }
    }
}

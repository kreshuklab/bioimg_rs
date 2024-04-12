use std::collections::{BTreeMap, HashMap, HashSet};

use bioimg_spec::rdf::model::{
    axis_size::{AxisSizeResolutionError, QualifiedAxisId, ResolvedAxisSize}, AnyAxisSize, AxisSizeReference, FixedAxisSize, ParameterizedAxisSize
};

pub trait ResolveExt{
    fn try_resolve(
        &mut self, size_map: &HashMap<QualifiedAxisId, ResolvedAxisSize>
    ) -> Result<ResolvedAxisSize, AxisSizeResolutionError>;
}

impl ResolveExt for FixedAxisSize{
    fn try_resolve(
        &mut self, size_map: &HashMap<QualifiedAxisId, ResolvedAxisSize>
    ) -> Result<ResolvedAxisSize, AxisSizeResolutionError> {
        Ok(ResolvedAxisSize::Fixed(self.clone()))
    }
}

impl ResolveExt for ParameterizedAxisSize{
    fn try_resolve(
        &mut self, size_map: &HashMap<QualifiedAxisId, ResolvedAxisSize>
    ) -> Result<ResolvedAxisSize, AxisSizeResolutionError> {
        Ok(ResolvedAxisSize::Parameterized(self.clone()))
    }
}

impl ResolveExt for AxisSizeReference{
    fn try_resolve(
        &mut self, size_map: &HashMap<QualifiedAxisId, ResolvedAxisSize>
    ) -> Result<ResolvedAxisSize, AxisSizeResolutionError> {
        Ok(size_map.get(&self.qualified_axis_id).unwrap()) //FIXME
    }
}

impl ResolveExt for AnyAxisSize{
    fn try_resolve(
        &mut self, size_map: &HashMap<QualifiedAxisId, ResolvedAxisSize>
    ) -> Result<ResolvedAxisSize, AxisSizeResolutionError> {
        match self {
            Self::Fixed(fixed) => fixed.try_resolve(size_map),
            Self::Parameterized(parameterized) => parameterized.try_resolve(size_map),
            Self::Reference(size_ref) => {
                let resolved = size_ref.resolve_with(size_map);
                *self = resolved.clone().into();
                Ok(resolved.clone()) //FIXME?
            },
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
    #[error("Resolve reference to {0} is unresolvable")]
    Unresolvable(QualifiedAxisId),
    #[error("Multiple axes with same ID: {0}")]
    DuplicateId(QualifiedAxisId),
}

impl SlotResolver {
    pub fn new(sizes: Vec<(QualifiedAxisId, AnyAxisSize)>) -> Result<Self, AxisSizeResolutionError> {
        let mut resolved_axes: HashMap<QualifiedAxisId, ResolvedAxisSize> = HashMap::with_capacity(sizes.len());
        let mut unresolved_axes: BTreeMap<QualifiedAxisId, AxisSizeReference> = BTreeMap::new();
        for (qual_id, inp_size) in sizes.into_iter() {
            let duplicate_detected = match inp_size {
                AnyAxisSize::Reference(size_ref) => {
                    matches!(unresolved_axes.insert(qual_id.clone(), size_ref.clone()), Some(_))
                }
                AnyAxisSize::Resolved(resolved_size) => {
                    matches!(resolved_axes.insert(qual_id.clone(), resolved_size.clone()), Some(_))
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
        if visited.contains(&current) {
            return Err(AxisSizeResolutionError::Loop(current));
        }
        visited.insert(current.clone());
        let Some(size_ref) = self.unresolved_axes.get(&current) else {
            return Err(AxisSizeResolutionError::Unresolvable(current));
        };
        let resolved = self.try_resolve(size_ref.qualified_axis_id.clone(), visited)?;
        self.unresolved_axes.remove(&current);
        self.resolved_axes.insert(current.clone(), resolved.clone());
        Ok(resolved)
    }

    fn step(mut self) -> Result<ResolverStatus, AxisSizeResolutionError> {
        Ok(match self.unresolved_axes.last_key_value() {
            Some((key, _)) => {
                self.try_resolve(key.clone(), HashSet::new())?;
                ResolverStatus::Resolving(self)
            }
            None => ResolverStatus::Done(self.resolved_axes),
        })
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

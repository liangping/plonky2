use plonky2::field::field_types::Field;
use plonky2::field::packed_field::PackedField;
use plonky2::iop::ext_target::ExtensionTarget;

#[derive(Debug, Copy, Clone)]
pub struct StarkEvaluationVars<'a, F, P>
where
    F: Field,
    P: PackedField<Scalar = F>,
{
    pub local_values: &'a [P],
    pub next_values: &'a [P],
    pub public_inputs: &'a [P::Scalar],
}

#[derive(Debug, Copy, Clone)]
pub struct StarkEvaluationTargets<
    'a,
    const D: usize,
    const COLUMNS: usize,
    const PUBLIC_INPUTS: usize,
> {
    pub local_values: &'a [ExtensionTarget<D>; COLUMNS],
    pub next_values: &'a [ExtensionTarget<D>; COLUMNS],
    pub public_inputs: &'a [ExtensionTarget<D>; PUBLIC_INPUTS],
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum ScaleRangeMode {
    #[serde(rename = "per_dataset")]
    PerDataset,
    #[serde(rename = "per_sample")]
    PerSample,
}


pub struct ScaleRangeDescr{
    /// Mode for computing percentiles.
    /// |     mode    |             description              |
    /// | ----------- | ------------------------------------ |
    /// | per_dataset | compute for the entire dataset       |
    /// | per_sample  | compute for each sample individually |
    mode: ScaleRangeMode,

    /// The subset of axes to normalize jointly.
    /// For example xy to normalize the two image axes for 2d data jointly
    // FIXME: axes: Annotated[AxesInCZYX, Field(examples=["xy"])]

    min_percentile: Annotated[Union[int, float], Interval(ge=0, lt=100)] = 0.0
    """The lower percentile used for normalization."""

    max_percentile: Annotated[Union[int, float], Interval(gt=1, le=100)] = 100.0
    """The upper percentile used for normalization
    Has to be bigger than `min_percentile`.
    The range is 1 to 100 instead of 0 to 100 to avoid mistakenly
    accepting percentiles specified in the range 0.0 to 1.0."""

    @model_validator(mode="after")
    def min_smaller_max(self, info: ValidationInfo) -> Self:
        if self.min_percentile >= self.max_percentile:
            raise ValueError(
                f"min_percentile {self.min_percentile} >= max_percentile"
                + f" {self.max_percentile}"
            )

        return self

    eps: Annotated[float, Interval(gt=0, le=0.1)] = 1e-6
    """Epsilon for numeric stability.
    `out = (tensor - v_lower) / (v_upper - v_lower + eps)`;
    with `v_lower,v_upper` values at the respective percentiles."""

    reference_tensor: Optional[TensorName] = None
    """Tensor name to compute the percentiles from. Default: The tensor itself.
    For any tensor in `inputs` only input tensor references are allowed.
    For a tensor in `outputs` only input tensor refereences are allowed if `mode: per_dataset`"""
}

const fn _default_min_percentile() -> f32 {
    0f32
}

const fn _default_max_percentile() -> f32 {
    100f32
}
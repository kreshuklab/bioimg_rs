use std::path::PathBuf;

use bioimg_spec::rdf::model as modelrdf;
use crate::widgets::{author_widget::AuthorWidget, Restore};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AuthorWidgetRawData{
    pub name_widget: String,
    pub affiliation_widget: Option<String>,
    pub email_widget: Option<String>,
    pub github_user_widget: Option<String>,
    pub orcid_widget: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CiteEntryWidgetRawData {
    pub citation_text_widget: String,
    pub doi_widget: Option<String>,
    pub url_widget: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MaintainerWidgetRawData {
    pub github_user_widget: String,
    pub affiliation_widget: Option<String>,
    pub email_widget: Option<String>,
    pub orcid_widget: Option<String>,
    pub name_widget: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum FileWidgetRawData{
    Empty,
    AboutToLoad{path: PathBuf},
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum LocalFileSourceWidgetRawData{
    Empty,
    AboutToLoad{path: String, inner_path: Option<String>}
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum FileSourceWidgetRawData{
    Local(LocalFileSourceWidgetRawData),
    Url(String),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ImageWidget2LoadingStateRawData{
    Empty,
    Forced{img_bytes: Vec<u8>}
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ImageWidget2RawData{
    pub file_source_widget: FileSourceWidgetRawData,
    pub loading_state: ImageWidget2LoadingStateRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpecialImageWidgetRawData{
    pub image_widget: ImageWidget2RawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum IconWidgetRawData{
    Emoji(String),
    Image(SpecialImageWidgetRawData),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CollapsibleWidgetRawData<Inner: Restore>{
    pub is_closed: bool,
    pub inner: Inner::RawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VersionWidgetRawData{
    pub raw: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CodeEditorWidgetRawData{
    pub raw: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BatchAxisWidgetRawData{
    pub description_widget: String,
    pub staging_allow_auto_size: bool,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum ChannelNamesModeRawData{
    Explicit,
    Pattern,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum AxisSizeModeRawData{
    Fixed,
    Reference,
    Parameterized,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ParameterizedAxisSizeWidgetRawData {
    pub staging_min: usize,
    pub staging_step: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AnyAxisSizeWidgetRawData {
    pub mode: AxisSizeModeRawData,

    pub staging_fixed_size: usize,
    pub staging_size_ref: AxisSizeReferenceWidgetRawData,
    pub staging_parameterized: ParameterizedAxisSizeWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct IndexAxisWidgetRawData {
    pub description_widget: String,
    pub size_widget: AnyAxisSizeWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AxisSizeReferenceWidgetRawData {
    pub staging_tensor_id: String,
    pub staging_axis_id: String,
    pub staging_offset: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChannelAxisWidgetRawData {
    pub description_widget: String,

    pub channel_names_mode_widget: ChannelNamesModeRawData,
    pub channel_extent_widget: usize,
    pub channel_name_prefix_widget: String,
    pub channel_name_suffix_widget: String,

    pub staging_explicit_names: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputSpaceAxisWidgetRawData {
    pub id_widget: String,
    pub description_widget: String,

    pub size_widget: AnyAxisSizeWidgetRawData,
    pub space_unit_widget: Option<modelrdf::SpaceUnit>,
    pub scale_widget: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputTimeAxisWidgetRawData {
    pub id_widget: String,
    pub description_widget: String,

    pub size_widget: AnyAxisSizeWidgetRawData,
    pub time_unit_widget: Option<modelrdf::TimeUnit>,
    pub scale_widget: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputAxisWidgetRawData {
    pub axis_type_widget: bioimg_spec::rdf::model::axes::AxisType,
    pub batch_axis_widget: BatchAxisWidgetRawData,
    pub channel_axis_widget: ChannelAxisWidgetRawData,
    pub index_axis_widget: IndexAxisWidgetRawData,
    pub space_axis_widget: InputSpaceAxisWidgetRawData,
    pub time_axis_widget: InputTimeAxisWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WeightsDescrBaseWidgetRawData{
    pub source_widget: FileSourceWidgetRawData,
    pub authors_widget: Option<Vec<CollapsibleWidgetRawData<AuthorWidget>>>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TorchscriptWeightsWidgetRawData{
    pub base_widget: WeightsDescrBaseWidgetRawData,
    pub pytorch_version_widget: VersionWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct JsonObjectEditorWidgetRawData{
    pub code_editor_widget: CodeEditorWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PytorchArchWidgetRawData{
    pub callable_widget: String,
    pub kwargs_widget: JsonObjectEditorWidgetRawData,
    pub import_from_widget: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PytorchStateDictWidgetRawData{
    pub base_widget: WeightsDescrBaseWidgetRawData,
    pub architecture_widget: PytorchArchWidgetRawData,
    pub version_widget: VersionWidgetRawData,
    pub dependencies_widget: Option<FileWidgetRawData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OnnxWeightsWidgetRawData{
    pub base_widget: WeightsDescrBaseWidgetRawData,
    pub opset_version_widget: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct KerasHdf5WeightsWidgetRawData{
    pub base_widget: WeightsDescrBaseWidgetRawData,
    pub tensorflow_version_widget: VersionWidgetRawData,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WeightsWidgetRawData{
    pub keras_weights_widget: Option<KerasHdf5WeightsWidgetRawData>,
    pub torchscript_weights_widget: Option<TorchscriptWeightsWidgetRawData>,
    pub pytorch_state_dict_widget: Option<PytorchStateDictWidgetRawData>,
    pub onnx_eights_widget: Option<OnnxWeightsWidgetRawData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProjectData1{
    pub staging_name: String,
    pub staging_description: String,
    // pub cover_images: Vec<>, // something from LoadingSate in image_widget_2
    // id?
    // pub staging_authors: Vec<CollapsibleWidget<StagingAuthor2>>,
    // pub attachments_widget: StagingVec<AttachmentsWidget>,
    // pub staging_citations: StagingVec<CollapsibleWidget<StagingCiteEntry2>>,
    // //config
    // pub staging_git_repo: StagingOpt<StagingUrl>,
    // pub icon_widget: StagingOpt<IconWidget>,
    // //links
    // pub staging_maintainers: StagingVec<CollapsibleWidget<StagingMaintainer>>,
    // pub staging_tags: StagingVec<StagingString<rdf::Tag>>,
    // pub staging_version: StagingOpt<VersionWidget>,

    // pub staging_documentation: CodeEditorWidget,
    // pub staging_license: SearchAndPickWidget<rdf::LicenseId>,
    // //badges
    // pub model_interface_widget: ModelInterfaceWidget,
    // ////
    // pub weights_widget: WeightsWidget,

    // pub notifications_widget: NotificationsWidget,
    // model_packing_status: PackingStatus,
}
